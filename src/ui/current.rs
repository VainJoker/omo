use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

use crate::app::App;
pub fn draw_curr<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let items: Vec<ListItem> = app
        .clone()
        .current
        .node
        .tc
        .into_keys()
        .into_iter()
        .map(|i| {
            let lines = vec![Spans::from(
                i.to_str().expect("cant convert into str").to_owned(),
            )];
            ListItem::new(lines).style(Style::default())
        })
        .collect();
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Current"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, area, &mut app.current.state);
}
