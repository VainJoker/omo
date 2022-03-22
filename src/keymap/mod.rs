use std::{io, path::Path};

use crossterm::event::{self, Event, KeyCode};
use log::debug;
use tui::{backend::Backend, Terminal};

use crate::{app::App, pop::Poptype, ui::draw};
//按键绑定
pub fn keymap<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    let mut app = app.clone();
    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match app.popup.show_popup {
                true => match key.code {
                    KeyCode::Esc => {
                        app.popup.show_popup = false;
                    }
                    KeyCode::Enter => {
                        app.popup.messages.push(app.popup.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        app.popup.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.popup.input.pop();
                    }
                    _ => {}
                },
                false => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::Char('/') => {
                        app.popup.poptype = Poptype::Search;
                        app.popup.show_popup = !app.popup.show_popup;
                    }
                    KeyCode::Char('-') => {
                        app.popup.poptype = Poptype::Delete;
                        app.popup.show_popup = !app.popup.show_popup;
                    }
                    KeyCode::Char('+') => {
                        app.popup.poptype = Poptype::Create;
                        app.popup.show_popup = !app.popup.show_popup;
                    }
                    KeyCode::Char('R') => {
                        app.popup.poptype = Poptype::Rename;
                        app.popup.show_popup = !app.popup.show_popup;
                    }
                    KeyCode::Char('h') => {
                        if app.current.node.current_path
                            != home::home_dir().expect("user's home_dir not found")
                            || app.current.node.current_path == Path::new("/root")
                        {
                            app = app.get_parapp();
                            debug!("Current Path is {:#?}", app.clone().get_item_path());
                        }
                    }
                    KeyCode::Char('l') => {
                        app = app.get_chiapp();
                        debug!("Current Path is {:#?}", app.clone().get_item_path());
                    }
                    KeyCode::Char('j') => {
                        app.current.next();
                        debug!("Current Path is {:#?}", app.clone().get_item_path());
                    }
                    KeyCode::Char('k') => {
                        app.current.previous();
                        debug!("Current Path is {:#?}", app.clone().get_item_path());
                    }
                    _ => {}
                },
            };
        }
    }
}
