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

#[derive(Clone, Debug)]
pub enum PanelSide {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum FileAction {
    Delete {
        path: String,
        tab: String,
        panel: PanelSide,
    },
    Rename {
        from: String,
        to: String,
        tab: String,
        panel: PanelSide,
    },
    Move {
        from: String,
        to: String,
        tab: String,
        panel: PanelSide,
    },
    Open {
        path: String,
        tab: String,
        panel: PanelSide,
    },
}

#[derive(Clone, Debug)]
pub enum DirectoryAction {
    Delete {
        path: String,
        tab: String,
        panel: PanelSide,
    },
    Rename {
        from: String,
        to: String,
        tab: String,
        panel: PanelSide,
    },
    Move {
        from: String,
        to: String,
        tab: String,
        panel: PanelSide,
    },
    Open {
        path: String,
        tab: String,
        panel: PanelSide,
    },
}
