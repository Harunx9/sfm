#[derive(Clone, Debug)]
pub struct AppState {
    pub left_tab: TabState,
    pub right_tab: TabState,
    pub app_exit: bool,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            left_tab: TabState::default(),
            right_tab: TabState::default(),
            app_exit: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TabState {
    pub title: String,
    pub items: Vec<FileSystemItem>,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
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
