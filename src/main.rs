use crate::{app::App, ui::run};
use log::{info, LevelFilter};

pub mod app;
pub mod item;
pub mod keymap;
pub mod node;
pub mod pop;
pub mod ui;
pub mod util;

fn main() {
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(LevelFilter::Debug);
    let app = App::new();
    info!(target: ""," Welcome 2 OMO !!!\n");
    run(app).expect("Can't draw the app");
}
