#[derive(Clone, Debug)]
pub struct PopUp {
    pub show_popup: bool,
    pub input: String,
    pub poptype: Poptype,
    pub messages: Vec<String>,
}

// 为popup划分类型,由于需要所有弹出式消息共用一个窗口，用来分类处理
#[derive(Clone, Debug)]
pub enum Poptype {
    Search,
    Create,
    Delete,
    Rename,
    Init,
}

impl Default for PopUp {
    fn default() -> PopUp {
        PopUp {
            input: String::new(),
            messages: Vec::new(),
            show_popup: false,
            poptype: Poptype::Init,
        }
    }
}
