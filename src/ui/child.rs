use std::path::Path;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

use crate::app::App;
use crate::util::get_content;
pub fn draw_chil<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let child_path = app.clone().get_item_path();
    //现在我们有了一个path,怎么获取path目录里的文件呢
    if Path::is_dir(&child_path) {
        let dir = get_content(child_path);
        let item: Vec<ListItem> = dir
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(
                    i.to_str().expect("cant convert into str").to_owned(),
                )];
                ListItem::new(lines).style(Style::default())
            })
            .collect();
        let items = List::new(item).block(Block::default().borders(Borders::ALL).title("Child"));
        f.render_widget(items, area);
    } else {
        let preview =
            Paragraph::new("Preview").block(Block::default().borders(Borders::ALL).title("Child"));
        f.render_widget(preview, area);
    }
}
