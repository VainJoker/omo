use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use std::path::Path;
use std::{collections::BTreeMap, env::current_dir, ffi::OsString, path::PathBuf};
use tui::backend::CrosstermBackend;
use tui::widgets::ListState;
use tui::Terminal;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Node {
    current_path: PathBuf,
    tc: BTreeMap<OsString, Vec<OsString>>,
    tp: BTreeMap<OsString, Vec<OsString>>,
}

#[derive(Clone, Debug)]
pub struct Item {
    state: ListState,
    node: Node,
}

impl Item {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            node: Node::new(),
        }
    }
    pub fn default(mut self) -> Self {
        self.state.select(Some(0));
        Self {
            node: self.node.default(),
            state: self.state,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= get_content(self.clone().node.current_path).len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    get_content(self.clone().node.current_path).len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

#[derive(Clone, Debug)]
pub struct App {
    current: Item,
    show_popup: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            // parent:Item::new().default(),
            // child:Item::new().default(),
            current: Item::new().default(),
            show_popup: false,
        }
    }
    pub fn get_parapp(&mut self) -> Self {
        // self.current.state.select(Some(0));
        let mut parent = Item::new();
        parent.node.current_path = self
            .current
            .node
            .current_path
            .parent()
            .unwrap()
            .to_path_buf();
        parent.node.set_tp();
        parent.node.set_tc();
        parent.state.select(Some(0));
        Self {
            current: parent,
            show_popup: false,
        }
    }
    pub fn get_chiapp(&mut self) -> Self {
        // self.current.state.select(Some(0));
        if !get_content(self.current.node.current_path.clone()).is_empty() {
            let mut child = Item::new();
            child.node.current_path = self.clone().get_child_path();
            child.node.set_tp();
            child.node.set_tc();
            child.state.select(Some(0));
            Self {
                current: child,
                show_popup: false,
            }
        } else {
            Self {
                current: self.clone().current,
                show_popup: false,
            }
        }
    }
    pub fn which_is_selected(self) -> PathBuf {
        let items: Vec<OsString> = self.current.node.tc.into_keys().collect();
        let selected_item = items
            .get(self.current.state.selected().expect("aaa"))
            .expect("bbb")
            .clone();
        PathBuf::from(selected_item)
    }

    pub fn get_child_path(self) -> PathBuf {
        let mut path_origin = self.clone().current.node.current_path;
        let path_add = self.clone().which_is_selected();
        path_origin.push(path_add);
        // println!("{:#?}",path_origin);
        path_origin
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::new(),
            tc: BTreeMap::new(),
            tp: BTreeMap::new(),
        }
    }
    pub fn default(&mut self) -> Self {
        self.set_current_path();
        self.set_tp();
        self.set_tc();
        Self {
            current_path: self.to_owned().current_path,
            tc: self.to_owned().tc,
            tp: self.to_owned().tp,
        }
    }

    pub fn set_tp(&mut self) {
        self.tp = BTreeMap::new();
        let mut parent: Vec<OsString> = Vec::new();
        for entry in WalkDir::new(self.current_path.parent().unwrap())
            .max_depth(1)
            .min_depth(1)
            .sort_by_file_name()
        {
            parent.push(entry.unwrap().file_name().to_os_string());
        }
        let c = OsString::from(self.current_path.file_name().clone().unwrap());
        self.tp.insert(c, parent);
    }

    pub fn set_tc(&mut self) {
        self.tc = BTreeMap::new();
        let child: Vec<OsString> = Vec::new();
        for entry in WalkDir::new(self.current_path.clone())
            .max_depth(1)
            .min_depth(1)
            .sort_by_file_name()
        {
            let mut child = child.clone();
            let entry = entry.unwrap();
            let path = entry.path();
            for child_entry in WalkDir::new(path)
                .max_depth(1)
                .min_depth(1)
                .sort_by_file_name()
            {
                child.push(child_entry.unwrap().file_name().to_os_string());
            }
            self.tc.insert(entry.file_name().to_os_string(), child);
        }
    }

    fn set_current_path(&mut self) {
        let mut current_path = PathBuf::new();
        let walker = WalkDir::new(current_dir().unwrap())
            .max_depth(0)
            .sort_by_file_name()
            .into_iter();
        for entry in walker {
            let a = entry.unwrap();
            current_path = a.path().to_path_buf()
        }
        self.current_path = current_path;
    }
}

pub fn get_content(path: PathBuf) -> Vec<OsString> {
    let walker = WalkDir::new(&path)
        .max_depth(1)
        .min_depth(1)
        .sort_by_file_name()
        .into_iter();
    let mut contents: Vec<OsString> = Vec::new();
    if Path::is_dir(&path) {
        for entry in walker {
            let content = entry.unwrap();
            contents.push(content.file_name().to_os_string());
        }
    }
    contents
}

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

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    if app.show_popup {
        let block = Block::default().title("PopUp").borders(Borders::ALL);
        let area = centered_rect(60, 20, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);
    }
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(30),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_pare(f, chunks[0], app);
    draw_curr(f, chunks[1], app);
    draw_chil(f, chunks[2], app);
}

fn draw_pare<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let mut items: Vec<ListItem> = Vec::new();
    let items_a = app.clone().current.node.tp.into_values().into_iter();
    for j in items_a {
        items = j
            .into_iter()
            .map(|i| {
                let lines = vec![Spans::from(i.to_str().unwrap().to_owned())];
                ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::DarkGray))
            })
            .collect();
    }
    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Parent"))
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    // We can now render the item list
    f.render_widget(items, area);
}

fn draw_curr<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
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
            let lines = vec![Spans::from(i.to_str().unwrap().to_owned())];
            ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::DarkGray))
        })
        .collect();
    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Current"))
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    // We can now render the item list
    f.render_stateful_widget(items, area, &mut app.current.state);
}

fn draw_chil<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let child_path = app.clone().get_child_path();
    //现在我们有了一个path,怎么获取path目录里的文件呢
    if Path::is_dir(&child_path) {
        let dir = get_content(child_path);
        // // Iterate through all elements in the `items` app and append some debug text to it.
        let item: Vec<ListItem> = dir
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(i.to_str().unwrap().to_owned())];
                ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::DarkGray))
            })
            .collect();
        // // let selected_dir = items.get(app.current_dir.state.selected().expect("aaa")).expect("bbb").clone();
        let items = List::new(item)
            .block(Block::default().borders(Borders::ALL).title("Child"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(items, area);
    } else {
        let preview =
            Paragraph::new("Preview").block(Block::default().borders(Borders::ALL).title("Child"));
        f.render_widget(preview, area);
    }
}

pub fn ui(app: App) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    keymap(&mut terminal, app).unwrap();
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn keymap<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    let mut app = app.clone();
    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Char('p') => app.show_popup = !app.show_popup,
                KeyCode::Char('h') => {
                    // app.current_dir.unselect();
                    app = app.get_parapp();
                }
                KeyCode::Char('l') => {
                    // app.current_dir.unselect();
                    app = app.get_chiapp();
                }
                KeyCode::Char('j') => app.current.next(),
                KeyCode::Char('k') => app.current.previous(),
                _ => {}
            }
        }
    }
}

fn main() {
    let app = App::new();
    ui(app).unwrap();
}