use std::fmt::Debug;
use std::path::{Path, PathBuf};

use tui::widgets::ListState;

use super::{
    actions::PanelSide,
    config::{icon_cfg::IconsConfig, Config},
    file_system::{file_system_item::FileSystemItem, DirInfo, FileSystem},
};

pub type TabIdx = usize;

#[derive(Clone, Debug)]
pub struct ChildProgramDesc {
    pub program_name: String,
    pub args: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AppState<TFileSystem: Clone + Debug + Default + FileSystem> {
    pub left_panel: PanelState<TFileSystem>,
    pub right_panel: PanelState<TFileSystem>,
    pub app_exit: bool,
    pub config: Config,
    pub child_program: Option<ChildProgramDesc>,
    pub modal: Option<ModalType>,
    pub file_system: TFileSystem,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> AppState<TFileSystem> {
    pub fn new(config: Config, file_system: TFileSystem) -> Self {
        let mut state = AppState::default();
        state.file_system = file_system;
        state.config = config;

        state
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> Default for AppState<TFileSystem> {
    fn default() -> Self {
        AppState {
            left_panel: PanelState::default(),
            right_panel: PanelState::default(),
            app_exit: false,
            config: Config::default(),
            child_program: None,
            modal: None,
            file_system: TFileSystem::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PanelState<TFileSystem: Clone + Debug + Default + FileSystem> {
    pub tabs: Vec<TabState<TFileSystem>>,
    pub is_focused: bool,
    pub current_tab: TabIdx,
    pub marker: std::marker::PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> Default for PanelState<TFileSystem> {
    fn default() -> Self {
        PanelState {
            tabs: vec![TabState::default()],
            is_focused: false,
            current_tab: 0,
            marker: std::marker::PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TabState<TFileSystem: Clone + Debug + Default + FileSystem> {
    pub name: String,
    pub icon: String,
    pub path: PathBuf,
    pub items: Vec<FileSystemItem>,
    pub selected: Vec<FileSystemItem>,
    pub tab_state: ListState,
    pub search_mode: bool,
    pub phrase: String,
    pub marker: std::marker::PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> Default for TabState<TFileSystem> {
    fn default() -> Self {
        TabState::with_dir(
            &Path::new("."),
            &TFileSystem::default(),
            &IconsConfig::default(),
        )
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> TabState<TFileSystem> {
    pub fn with_dir(dir_path: &Path, file_system: &TFileSystem, icons: &IconsConfig) -> Self {
        let dir_info = DirInfo::new(&dir_path).unwrap();
        let items = file_system.list_dir(&dir_info.path, icons);
        TabState {
            name: dir_info.name.clone(),
            icon: icons.get_dir_icon(dir_info.name.clone()),
            path: dir_info.path.clone(),
            items,
            selected: Vec::new(),
            tab_state: ListState::default(),
            search_mode: false,
            phrase: String::from(""),
            marker: std::marker::PhantomData,
        }
    }

    pub fn filtered_items(&self) -> Vec<&FileSystemItem> {
        if self.phrase.is_empty() {
            self.items.iter().collect()
        } else {
            self.items
                .iter()
                .filter(|item| {
                    item.get_name()
                        .to_lowercase()
                        .contains(&self.phrase.to_lowercase())
                })
                .collect()
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
    DeleteDirWithContent {
        panel_side: PanelSide,
        panel_tab: TabIdx,
        path: PathBuf,
    },
}
