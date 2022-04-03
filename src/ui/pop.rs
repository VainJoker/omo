use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

use crate::app::App;
use crate::pop::Poptype;
//居中处理popup
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
pub fn draw_pop<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    //处理popup对应的事件，可变成函数单独拎出来
    match app.popup.poptype {
        Poptype::Search => print_pop(f, app, String::from("Input Which you want to search")),
        Poptype::Create => print_pop(f, app, String::from("Input which you want to create")),
        Poptype::Delete => print_pop(f, app, String::from("Input Y to delete")),
        Poptype::Rename => print_pop(f, app, String::from("Input the name you want to rename")),
        Poptype::Init => {}
    }
}

fn print_pop<B>(f: &mut Frame<B>, app: &mut App, title: String)
where
    B: Backend,
{
    if app.popup.show_popup {
        let area = centered_rect(30, 10, f.size());
        f.render_widget(Clear, area); //this clears out the background
        let input = Paragraph::new(app.popup.input.as_ref())
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(input, area);
        f.set_cursor(
            // Put cursor past the end of the input text
            area.x + app.popup.input.width() as u16 + 1,
            // Move one line down, from the border to the input line
            area.y + 1,
        );
    }
}
