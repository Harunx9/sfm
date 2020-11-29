#[derive(Clone, Debug)]
pub struct State {
    pub left_tab: Tab,
    pub right_tab: Tab,
    pub app_exit: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            left_tab: Tab::default(),
            right_tab: Tab::default(),
            app_exit: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tab {
    pub title: String,
    pub items: Vec<FileSystemItem>,
}

impl Default for Tab {
    fn default() -> Self {
        Tab {
            title: String::new(),
            items: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileSystemItem {
    Directory(DirectoryItem),
    File(FileItem),
}

#[derive(Clone, Debug)]
pub struct DirectoryItem {
    is_visible: bool,
    name: String,
    path: String,
    icon: char,
}

#[derive(Clone, Debug)]
pub struct FileItem {
    is_visible: bool,
    name: String,
    path: String,
    icon: char,
}
