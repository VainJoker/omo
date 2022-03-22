use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_logger::TuiLoggerWidget;
pub fn draw_logs<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let log = TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Blue))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Cyan))
        .block(Block::default().title("Logs").borders(Borders::ALL));
    f.render_widget(log, area);
}
