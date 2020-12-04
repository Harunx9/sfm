use super::file_system::FileSystemItem;

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
    pub name: String,
    pub path: String,
    pub items: Vec<FileSystemItem>,
    pub is_focused: bool,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            name: String::default(),
            path: String::default(),
            is_focused: false,
            items: Vec::new(),
        }
    }
}
