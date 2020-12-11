#[derive(Clone, Debug)]
pub enum FileManagerActions {
    File(FileAction),
    Directory(DirectoryAction),
    App(AppAction),
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
    Select,
}

#[derive(Clone, Debug)]
pub enum FileAction {
    Delete { path: String },
    Rename { from: String, to: String },
    Move { from: String, to: String },
    Open { path: String },
}

#[derive(Clone, Debug)]
pub enum DirectoryAction {
    Delete { path: String },
    Rename { from: String, to: String },
    Move { from: String, to: String },
    Open { path: String },
    Navigate { to: String },
}
