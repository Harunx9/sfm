use std::fs;

use super::file_system::{
    directory::{get_items_from_dir, DirInfo},
    FileSystemItem,
};

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
pub struct PaneState {
    tabs: Vec<TabState>,
    is_focused: bool,
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
        TabState::from_dir(".")
    }
}

impl TabState {
    pub fn from_dir(dir_path: &str) -> Self {
        let dir_info = DirInfo::new(dir_path).unwrap();
        let items = get_items_from_dir(dir_path);
        TabState {
            name: dir_info.name,
            path: dir_info.path,
            items,
            is_focused: false,
        }
    }
}
