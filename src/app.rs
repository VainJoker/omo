use crate::item::Item;
use crate::pop::PopUp;
use crate::util::get_content;
use std::ffi::OsString;
use std::path::PathBuf;
// App 三个文件窗口,一个log窗口，一个popup窗口
#[derive(Clone, Debug)]
pub struct App {
    pub current: Item,
    pub popup: PopUp,
}

impl App {
    pub async fn new() -> Self {
        Self {
            current: Item::new().default().await,
            popup: PopUp::default(),
        }
    }

    //移动逻辑
    pub async fn get_parapp(&mut self) -> Self {
        let mut parent = Item::new();
        match self.current.node.current_path.parent() {
            Some(i) => {
                parent.node.current_path = i.to_path_buf();
                parent.node.set_tp().await;
                parent.node.set_tc().await;
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
                    popup: PopUp::default(),
                }
            }
            None => Self {
                current: self.clone().current,
                popup: PopUp::default(),
            },
        }
    }

    //移动逻辑
    pub async fn get_chiapp(&mut self) -> Self {
        //如果是空文件夹不许移动
        if !get_content(self.clone().get_item_path()).is_empty() {
            let mut child = Item::new();
            child.node.current_path = self.clone().get_item_path();
            child.node.set_tp().await;
            child.node.set_tc().await;
            let file_index: usize = 0;
            child.state.select(Some(file_index));
            Self {
                current: child,
                popup: PopUp::default(),
            }
        } else {
            Self {
                current: self.clone().current,
                popup: PopUp::default(),
            }
        }
    }
    //通过移动当前路径，获取目录
    //对于当前路径，非根，父路径有且仅有一个,然而父目录的兄弟目录都需要读取,所以设置node的toparent（tp）这个BTreeMap为一个string对应一个vec
    //至于子目录，是一个多对多的关系，当前路径下有许多文件夹，许多的item,我需要把所有item的子文件夹存起来，所以设置node的tochild（tc）这个BTreeMap为一个vec对应一个vec
    //因为这导致当存在许多的条目时，会造成略微卡顿情况
    //todo!需要处理

    //获取被选中的条目
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

    //获取被选中的目录的路径
    pub fn get_item_path(self) -> PathBuf {
        let mut path_origin = self.clone().current.node.current_path;
        let path_add = self.clone().which_is_selected();
        path_origin.push(path_add);
        path_origin
    }
}
