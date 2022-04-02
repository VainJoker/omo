use crate::util::get_content;
use tui::widgets::ListState;

use crate::node::Node;
#[derive(Clone, Debug)]
pub struct Item {
    pub state: ListState,
    pub node: Node,
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
        if !get_content(self.clone().node.current_path).is_empty() {
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
    }

    pub fn previous(&mut self) {
        if !get_content(self.clone().node.current_path).is_empty() {
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
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }


}

