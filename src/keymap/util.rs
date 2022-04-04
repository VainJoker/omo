use std::{ffi::OsString, fs};

use log::debug;

use crate::app::App;
pub fn create_dir(mut app: App) {
    let create_dir_name = app.current.node.current_path.to_str().unwrap().to_owned()
        + "/"
        + &app.popup.message.clone();
    debug!("{create_dir_name}");
    match fs::create_dir(create_dir_name) {
        Ok(_) => app.current.node.set_tc(),
        Err(e) => debug!("{}", e),
    }
}

pub fn create_file(mut app: App) {
    let create_file_name = app.current.node.current_path.to_str().unwrap().to_owned()
        + "/"
        + &app.popup.message.clone();
    debug!("{create_file_name}");
    match fs::File::create(create_file_name) {
        Ok(_) => app.current.node.set_tc(),
        Err(e) => debug!("{}", e),
    }
}

pub fn search_file(mut app: App) {
    let search_name = app.popup.message.clone();
    let items: Vec<OsString> = app.current.node.tc.clone().into_keys().collect();
    if items.contains(&OsString::from(&search_name)) {
        app.current.state.select(
            items
                .iter()
                .position(|x| x == &OsString::from(search_name.clone())),
        )
    } else {
        debug!("Do not have the item,search again")
    }
}

pub fn delete_file(mut app: App) {
    if app.popup.message.clone() == "Y" {
        if app.clone().get_item_path().is_dir() {
            match std::fs::remove_dir_all(app.clone().get_item_path()) {
                Ok(_) => {
                    debug!("you have delete {:#?}", app.clone().which_is_selected());
                    app.current.node.set_tc();
                }
                Err(e) => debug!("{}", e),
            }
        } else {
            match std::fs::remove_file(app.clone().get_item_path()) {
                Ok(_) => {
                    debug!("you have delete {:#?}", app.clone().which_is_selected());
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

pub fn rename_file(mut app: App) {
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
