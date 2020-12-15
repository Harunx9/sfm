use tui::widgets::ListState;

use super::{
    config::{Config, IconsConfig},
    file_system::{
        directory::{get_items_from_dir, DirInfo},
        FileSystemItem,
    },
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub left_panel: PanelState,
    pub right_panel: PanelState,
    pub app_exit: bool,
    pub config: Config,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            left_panel: PanelState::default(),
            right_panel: PanelState::default(),
            app_exit: false,
            config: Config::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PanelState {
    pub tabs: Vec<TabState>,
    pub is_focused: bool,
    pub current_tab: usize,
}

impl Default for PanelState {
    fn default() -> Self {
        PanelState {
            tabs: vec![TabState::default()],
            is_focused: false,
            current_tab: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TabState {
    pub name: String,
    pub path: String,
    pub items: Vec<FileSystemItem>,
    pub selected: Vec<usize>,
    pub tab_state: ListState,
}

impl Default for TabState {
    fn default() -> Self {
        TabState::with_dir(".", &IconsConfig::default())
    }
}

impl TabState {
    pub fn with_dir(dir_path: &str, icons: &IconsConfig) -> Self {
        let dir_info = DirInfo::new(dir_path).unwrap();
        let items = get_items_from_dir(dir_path, icons);
        TabState {
            name: dir_info.name,
            path: dir_info.path,
            items,
            selected: Vec::new(),
            tab_state: ListState::default(),
        }
    }
}
