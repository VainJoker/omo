use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};
use log::debug;

use crate::{app::App, pop::Poptype};

pub fn keybinding(mut app: App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            return;
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
            if app.current.node.current_path != home::home_dir().expect("user's home_dir not found")
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
    }
}
