use std::{ffi::OsString, fs, io, path::Path};

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
                true => {
                    match key.code {
                        KeyCode::Esc => {
                            app.popup.show_popup = false;
                        }
                        KeyCode::Enter => {
                            app.popup.message = app.popup.input.clone();
                            match app.popup.poptype {
                                Poptype::Create => {
                                    let create_dir_name =
                                        app.current.node.current_path.to_str().unwrap().to_owned()
                                            + "/"
                                            + &app.popup.message.clone();
                                    debug!("{create_dir_name}");
                                    fs::create_dir(create_dir_name).unwrap();
                                    app.current.node.set_tc();
                                }
                                Poptype::Search => {
                                    let search_name = app.popup.message.clone();
                                    let items: Vec<OsString> =
                                        app.current.node.tc.clone().into_keys().collect();
                                    if items.contains(&OsString::from(&search_name)) {
                                        app.current.state.select(items.iter().position(|x| {
                                            x == &OsString::from(search_name.clone())
                                        }))
                                    } else {
                                        debug!("Do not have the item,search again")
                                    }
                                }
                                Poptype::Delete => {}
                                _ => {}
                            }
                            app.popup.input.clear();
                            app.popup.message.clear();
                            app.popup.show_popup = false;
                        }
                        KeyCode::Char(c) => {
                            app.popup.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.popup.input.pop();
                        }
                        _ => {}
                    }
                }
                false => match key.code {
                    KeyCode::Char('q') => {
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
                        app.popup.message.clear();
                        app.popup.show_popup = !app.popup.show_popup;
                        app.popup.poptype = Poptype::Create;
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
