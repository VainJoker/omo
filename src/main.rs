use log::{info, LevelFilter};

use crate::{app::App, ui::run};

pub mod app;
pub mod item;
pub mod keymap;
pub mod node;
pub mod pop;
pub mod ui;
pub mod util;

#[tokio::main]
async fn main() {
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(LevelFilter::Debug);
    let app = App::new().await;
    info!("Welcome 2 OMO !!!\n");
    run(app).await.expect("Can't draw the app");
}
