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
    FocusLeft,
    FocusRight,
}

#[derive(Clone, Debug)]
pub enum PanelAction {
    Next,
    Previous,
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
pub enum FileAction {
    Delete {
        path: PathBuf,
        tab: TabIdx,
        panel: PanelSide,
    },
    Rename {
        from: PathBuf,
        to: PathBuf,
        tab: TabIdx,
        panel: PanelSide,
    },
    Move {
        from: PathBuf,
        to: PathBuf,
        tab: TabIdx,
        panel: PanelSide,
    },
    Open {
        path: PathBuf,
        tab: TabIdx,
        panel: PanelSide,
    },
}

#[derive(Clone, Debug)]
pub enum DirectoryAction {
    Delete {
        path: PathBuf,
        tab: TabIdx,
        panel: PanelSide,
    },
    Rename {
        from: PathBuf,
        to: PathBuf,
        tab: TabIdx,
        panel: PanelSide,
    },
    Move {
        from: PathBuf,
        to: PathBuf,
        tab: TabIdx,
        panel: PanelSide,
    },
    Open {
        path: PathBuf,
        tab: TabIdx,
        panel: PanelSide,
    },
}
