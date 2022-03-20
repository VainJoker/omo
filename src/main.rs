use std::collections::BTreeMap;
use std::env::current_dir;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::{debug, info, LevelFilter};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};
use tui::{Frame, Terminal};
use tui_logger::TuiLoggerWidget;
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
            current: Item::new().default(),
            show_popup: false,
        }
    }

    pub fn get_parapp(&mut self) -> Self {
        let mut parent = Item::new();
        match self.current.node.current_path.parent() {
            Some(i) => {
                parent.node.current_path = i.to_path_buf();
                parent.node.set_tp();
                parent.node.set_tc();
                let mut file_index: usize = 0;
                match self.current.node.current_path.file_name() {
                    Some(j) => {
                        file_index = self
                            .current
                            .node
                            .tp
                            .get(j)
                            .unwrap()
                            .binary_search(&j.to_os_string())
                            .unwrap();
                    }
                    None => {}
                };
                parent.state.select(Some(file_index));
                Self {
                    current: parent,
                    show_popup: false,
                }
            }
            None => Self {
                current: self.clone().current,
                show_popup: false,
            },
        }
    }

    pub fn get_chiapp(&mut self) -> Self {
        if !get_content(self.clone().get_child_path()).is_empty() {
            let mut child = Item::new();
            child.node.current_path = self.clone().get_child_path();
            child.node.set_tp();
            child.node.set_tc();
            let file_index: usize = 0;
            child.state.select(Some(file_index));
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
        let mut selected_item = &OsString::new();
        match self.current.state.selected() {
            Some(i) => match items.get(i) {
                Some(j) => {
                    selected_item = j;
                }
                None => {}
            },
            None => {}
        };
        PathBuf::from(selected_item)
    }

    pub fn get_child_path(self) -> PathBuf {
        let mut path_origin = self.clone().current.node.current_path;
        let path_add = self.clone().which_is_selected();
        path_origin.push(path_add);
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
        match self.current_path.parent() {
            Some(i) => {
                for entry in WalkDir::new(i)
                    .max_depth(1)
                    .min_depth(1)
                    .follow_links(true)
                    .sort_by_file_name()
                {
                    match entry {
                        Ok(entry) => parent.push(entry.file_name().to_os_string()),
                        Err(_) => {}
                    };
                }
                let c = OsString::from(
                    self.current_path
                        .file_name()
                        .clone()
                        .expect("get file_name error"),
                );
                self.tp.insert(c, parent);
            }
            None => {}
        }
    }

    pub fn set_tc(&mut self) {
        self.tc = BTreeMap::new();
        let child: Vec<OsString> = Vec::new();
        for entry in WalkDir::new(self.current_path.clone())
            .max_depth(1)
            .min_depth(1)
            .follow_links(true)
            .sort_by_file_name()
        {
            let mut child = child.clone();
            let mut path = self.clone().current_path;
            match &entry {
                Ok(entry) => {
                    path = entry.path().to_path_buf();
                }
                Err(_) => {}
            };
            for child_entry in WalkDir::new(path)
                .max_depth(1)
                .min_depth(1)
                .follow_links(true)
                .sort_by_file_name()
            {
                match child_entry {
                    Ok(c) => child.push(c.file_name().to_os_string()),
                    Err(_) => {}
                }
            }
            match &entry {
                Ok(entry) => {
                    let file_name = entry.file_name().to_os_string();
                    self.tc.insert(file_name.to_os_string(), child);
                }
                Err(_) => {}
            };
        }
    }

    fn set_current_path(&mut self) {
        let mut current_path = PathBuf::new();
        let walker = WalkDir::new(current_dir().expect("get current_dir error"))
            .max_depth(0)
            .follow_links(true)
            .sort_by_file_name()
            .into_iter();
        for entry in walker {
            match entry {
                Ok(entry) => current_path = entry.path().to_path_buf(),
                Err(_) => {}
            }
        }
        self.current_path = current_path;
    }
}

pub fn get_content(path: PathBuf) -> Vec<OsString> {
    let walker = WalkDir::new(&path)
        .max_depth(1)
        .min_depth(1)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();
    let mut contents: Vec<OsString> = Vec::new();
    if Path::is_dir(&path) {
        for entry in walker {
            match entry {
                Ok(entry) => {
                    let content = entry;
                    contents.push(content.file_name().to_os_string());
                }
                Err(_) => {}
            };
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
    draw_pare(f, upsidechunk[0], app);
    draw_curr(f, upsidechunk[1], app);
    draw_chil(f, upsidechunk[2], app);
    draw_logs(f, mainchunk[1]);
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

fn draw_chil<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let child_path = app.clone().get_child_path();
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

fn draw_logs<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let log = TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Blue))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .title("Logs")
                .borders(Borders::ALL),
        );
    f.render_widget(log, area);
}

pub fn ui(app: App) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    keymap(&mut terminal, app).expect("keymap error");
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
                    if app.current.node.current_path
                        != home::home_dir().expect("user's home_dir not found")
                        || app.current.node.current_path == Path::new("/root")
                    {
                        app = app.get_parapp();
                        debug!("Current Path is {:#?}", app.current.node.current_path);
                    }
                }
                KeyCode::Char('l') => {
                    app = app.get_chiapp();
                    debug!("Current Path is {:#?}", app.current.node.current_path);
                }
                KeyCode::Char('j') => {
                    debug!("Next");

                    app.current.next();
                }
                KeyCode::Char('k') => {
                    debug!("Previous");

                    app.current.previous();
                }
                _ => {}
            }
        }
    }
}

fn main() {
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);
    let app = App::new();
    info!("Welcome 2 OMO !!!\n");
    ui(app).expect("Can't draw the ui");
}
