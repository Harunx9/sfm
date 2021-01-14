use std::path::{Path, PathBuf};

use tui::widgets::ListState;

use super::{
    actions::PanelSide,
    config::{icon_cfg::IconsConfig, Config},
    file_system::{
        directory::{get_items_from_dir, DirInfo},
        FileSystemItem,
    },
};

pub type TabIdx = usize;

#[derive(Clone, Debug)]
pub struct ChildProgramDesc {
    pub program_name: String,
    pub args: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub left_panel: PanelState,
    pub right_panel: PanelState,
    pub app_exit: bool,
    pub config: Config,
    pub child_program: Option<ChildProgramDesc>,
    pub modal: Option<ModalType>,
}

impl AppState {
    pub fn with_config(config: Config) -> Self {
        let mut state = AppState::default();
        state.config = config;

        state
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            left_panel: PanelState::default(),
            right_panel: PanelState::default(),
            app_exit: false,
            config: Config::default(),
            child_program: None,
            modal: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PanelState {
    pub tabs: Vec<TabState>,
    pub is_focused: bool,
    pub current_tab: TabIdx,
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
    pub icon: String,
    pub path: PathBuf,
    pub items: Vec<FileSystemItem>,
    pub selected: Vec<usize>,
    pub tab_state: ListState,
}

impl Default for TabState {
    fn default() -> Self {
        TabState::with_dir(&Path::new("."), &IconsConfig::default())
    }
}

impl TabState {
    pub fn with_dir(dir_path: &Path, icons: &IconsConfig) -> Self {
        let dir_info = DirInfo::new(dir_path).unwrap();
        let items = get_items_from_dir(dir_path, icons);
        TabState {
            name: dir_info.name.clone(),
            icon: icons.get_dir_icon(dir_info.name.clone()),
            path: dir_info.path.clone(),
            items,
            selected: Vec::new(),
            tab_state: ListState::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ModalType {
    RenameModal {
        panel_side: PanelSide,
        panel_tab: TabIdx,
        item: FileSystemItem,
    },
    CreateModal {
        item_index: Option<usize>,
        panel_side: PanelSide,
        panel_tab: TabIdx,
        panel_tab_path: PathBuf,
    },
    ErrorModal(String),
}
