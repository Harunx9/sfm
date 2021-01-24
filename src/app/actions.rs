use std::path::PathBuf;

use super::state::{ModalType, TabIdx};

#[derive(Clone, Debug)]
pub enum FileManagerActions {
    File(FileAction),
    Directory(DirectoryAction),
    Symlink(SymlinkAction),
    App(AppAction),
    Panel(PanelAction),
    Tab(TabAction),
}

#[derive(Clone, Debug)]
pub enum AppAction {
    Exit,
    ChildProgramClosed,
    FocusLeft,
    FocusRight,
    ShowModal(ModalType),
    CloseModal,
}

#[derive(Clone, Debug)]
pub enum PanelAction {
    Next { panel: PanelSide },
    Previous { panel: PanelSide },
    CloseTab { tab: TabIdx, panel: PanelSide },
}

#[derive(Clone, Debug)]
pub enum TabAction {
    Next,
    Previous,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PanelSide {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct PanelInfo {
    pub path: PathBuf,
    pub tab: TabIdx,
    pub side: PanelSide,
}

impl PartialEq for PanelInfo {
    fn eq(&self, other: &PanelInfo) -> bool {
        self.side == other.side && self.tab == other.tab
    }
}

#[derive(Clone, Debug)]
pub enum FileAction {
    Delete { panel: PanelInfo },
    Rename { from: PanelInfo, to: PanelInfo },
    Move { from: PanelInfo, to: PanelInfo },
    Open { panel: PanelInfo },
    Create { file_name: String, panel: PanelInfo },
}

#[derive(Clone, Debug)]
pub enum DirectoryAction {
    DeleteWithContent { panel: PanelInfo },
    Delete { panel: PanelInfo, is_empty: bool },
    Rename { from: PanelInfo, to: PanelInfo },
    Move { from: PanelInfo, to: PanelInfo },
    Open { panel: PanelInfo, in_new_tab: bool },
    Create { dir_name: String, panel: PanelInfo },
}

#[derive(Clone, Debug)]
pub enum SymlinkAction {
    Delete {
        panel: PanelInfo,
    },
    Open {
        panel: PanelInfo,
        in_new_tab: bool,
    },
    Create {
        symlink_path: PathBuf,
        panel: PanelInfo,
    },
}
