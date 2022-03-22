use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

use crate::app::App;
pub fn draw_pare<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let mut items: Vec<ListItem> = Vec::new();
    let items_a = app.clone().current.node.tp.into_values().into_iter();
    for j in items_a {
        items = j
            .into_iter()
            .map(|i| {
                let lines = vec![Spans::from(
                    i.to_str().expect("cant convert into str").to_owned(),
                )];
                ListItem::new(lines).style(Style::default())
            })
            .collect();
    }
    let items = List::new(items).block(Block::default().borders(Borders::ALL).title("Parent"));
    f.render_widget(items, area);
}
