pub mod keybindings;
pub mod util;

use std::io;

use crate::keymap::keybindings::keybinding;

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{app::App, pop::Poptype, ui::draw};

use self::util::{create_dir, create_file, delete_file, rename_file, search_file};

//按键绑定
pub fn keymap<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw(f, &mut app.clone()))?;
        key_match(app.clone()).unwrap();
    }
}

pub fn key_match(app: App) -> io::Result<()> {
    let mut app = app.clone();
    if let Event::Key(key) = event::read()? {
        match app.popup.show_popup {
            true => match key.code {
                KeyCode::Esc => {
                    app.popup.show_popup = false;
                }
                KeyCode::Enter => {
                    enter_press(app);
                }
                KeyCode::Char(c) => {
                    app.popup.input.push(c);
                }
                KeyCode::Backspace => {
                    app.popup.input.pop();
                }
                _ => {}
            },
            false => keybinding(app, key),
        }
    };
    Ok(())
}

pub fn enter_press(mut app: App) {
    app.popup.message = app.popup.input.clone();
    match app.popup.poptype {
        Poptype::CreateDir => {
            create_dir(app.clone());
        }
        Poptype::CreateFile => {
            create_file(app.clone());
        }
        //怎么输入几个字符去匹配
        Poptype::Search => {
            search_file(app.clone());
        }
        Poptype::Delete => {
            delete_file(app.clone());
        }
        Poptype::Rename => {
            rename_file(app.clone());
        }
        _ => {}
    }
    app.popup.input.clear();
    app.popup.message.clear();
    app.popup.show_popup = false;
}
