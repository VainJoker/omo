use std::error::Error;
use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::{Frame, Terminal};

use crate::app::App;
use crate::keymap::keymap;

use self::child::draw_chil;
use self::current::draw_curr;
use self::log::draw_logs;
use self::parent::draw_pare;
use self::pop::draw_pop;

pub mod child;
pub mod current;
pub mod log;
pub mod parent;
pub mod pop;
//tui绘制界面
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let mainchunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
        .split(f.size());
    let upsidechunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(30),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(mainchunk[0]);
    //父文件夹
    draw_pare(f, upsidechunk[0], app);
    //当前文件夹
    draw_curr(f, upsidechunk[1], app);
    //子文件夹
    draw_chil(f, upsidechunk[2], app);
    //log
    draw_logs(f, mainchunk[1]);
    //pop
    draw_pop(f, app);
}

//运行起来了
pub async fn run(app: App) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    keymap(&mut terminal, app).await.unwrap();
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
