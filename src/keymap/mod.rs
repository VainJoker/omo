use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

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
                                Poptype::CreateDir => {
                                    let create_dir_name =
                                        app.current.node.current_path.to_str().unwrap().to_owned()
                                            + "/"
                                            + &app.popup.message.clone();
                                    debug!("{create_dir_name}");
                                    match fs::create_dir(create_dir_name) {
                                        Ok(_) => app.current.node.set_tc(),
                                        Err(e) => debug!("{}", e),
                                    }
                                }
                                Poptype::CreateFile => {
                                    let create_file_name =
                                        app.current.node.current_path.to_str().unwrap().to_owned()
                                            + "/"
                                            + &app.popup.message.clone();
                                    debug!("{create_file_name}");
                                    match fs::File::create(create_file_name) {
                                        Ok(_) => app.current.node.set_tc(),
                                        Err(e) => debug!("{}", e),
                                    }
                                }
                                //输入几个字符去匹配
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
                                Poptype::Delete => {
                                    if app.popup.message.clone() == "Y" {
                                        if app.clone().get_item_path().is_dir() {
                                            match std::fs::remove_dir_all(
                                                app.clone().get_item_path(),
                                            ) {
                                                Ok(_) => {
                                                    debug!(
                                                        "you have delete {:#?}",
                                                        app.clone().which_is_selected()
                                                    );
                                                    app.current.node.set_tc();
                                                }
                                                Err(e) => debug!("{}", e),
                                            }
                                        } else {
                                            match std::fs::remove_file(app.clone().get_item_path())
                                            {
                                                Ok(_) => {
                                                    debug!(
                                                        "you have delete {:#?}",
                                                        app.clone().which_is_selected()
                                                    );
                                                    app.current.node.set_tc();
                                                }
                                                Err(e) => debug!("{}", e),
                                            }
                                        }
                                    } else if app.popup.message.clone() == "N" {
                                        app.popup.show_popup = false;
                                    } else {
                                        debug!("you have input wrong")
                                    }
                                }
                                Poptype::Rename => {
                                    let rename_name = app.popup.message.clone();
                                    match std::fs::rename(
                                        app.clone().get_item_path(),
                                        app.clone()
                                            .get_item_path()
                                            .parent()
                                            .expect("dont have the parent")
                                            .to_str()
                                            .expect("cant be str")
                                            .to_owned()
                                            + "/"
                                            + &rename_name,
                                    ) {
                                        Ok(_) => app.current.node.set_tc(),
                                        Err(e) => debug!("{}", e),
                                    }
                                }
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
                        app.popup.poptype = Poptype::CreateDir;
                    }
                    KeyCode::Char('N') => {
                        app.popup.message.clear();
                        app.popup.show_popup = !app.popup.show_popup;
                        app.popup.poptype = Poptype::CreateFile;
                    }
                    KeyCode::Char('R') => {
                        app.popup.poptype = Poptype::Rename;
                        app.popup.show_popup = !app.popup.show_popup;
                    }
                    KeyCode::Char('Y') => {
                        // let copy_name = app
                        //     .clone()
                        //     .get_item_path()
                        //     .to_owned();
                        // app.popup.message = copy_name.to_str().unwrap().to_string();
                        // debug!("{:#?}", app.popup.message);
                    }
                    KeyCode::Char('P') => {
                        // debug!("{:#?}", app.popup.message);
                        // let parse_name =
                        //     app.clone().get_item_path().parent().unwrap().to_str().unwrap().to_owned()
                        //     +"/"
                        //     + PathBuf::from(app.popup.message.clone()).file_name().unwrap().to_str().unwrap();
                        // debug!("{:#?}", parse_name);
                        // std::fs::copy(app.popup.message.clone(), parse_name).unwrap();
                    }
                    //咋搞呢！！！？？？
                    KeyCode::Char('S') => {
                        // return Ok(());
                    }
                    KeyCode::Char('h') => {
                        if app.current.node.current_path
                            != home::home_dir().expect("user's home_dir not found")
                            || app.current.node.current_path == Path::new("/root")
                        {
                            app = app.get_parapp();
                            debug!("Current Path is {:#?}", app.clone().get_item_path());
                            // debug!("{:?}", app.clone().get_item_path().metadata());
                        }
                    }
                    KeyCode::Char('l') => {
                        app = app.get_chiapp();
                        debug!("Current Path is {:#?}", app.clone().get_item_path());
                        // debug!("{:?}", app.clone().get_item_path().metadata());
                    }
                    KeyCode::Char('j') => {
                        app.current.next();
                        debug!("Current Path is {:#?}", app.clone().get_item_path());
                        // debug!("{:?}", app.clone().get_item_path().metadata());
                    }
                    KeyCode::Char('k') => {
                        app.current.previous();
                        debug!("Current Path is {:#?}", app.clone().get_item_path());
                        // debug!("{:?}", app.clone().get_item_path().metadata());
                    }
                    _ => {}
                },
            };
        }
    }
}
