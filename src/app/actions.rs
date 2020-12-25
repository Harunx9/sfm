use std::path::PathBuf;

use super::state::TabIdx;

#[derive(Clone, Debug)]
pub enum FileManagerActions {
    File(FileAction),
    Directory(DirectoryAction),
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
    Delete { panel: PanelInfo },
    Rename { from: PanelInfo, to: PanelInfo },
    Move { from: PanelInfo, to: PanelInfo },
    Open { panel: PanelInfo, in_new_tab: bool },
    Create { dir_name: String, panel: PanelInfo },
}
