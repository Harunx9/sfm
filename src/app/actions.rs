#[derive(Clone, Debug)]
pub enum FrActions {
    File(FileAction),
    Directory(DirectoryAction),
    App(AppActions),
}

#[derive(Clone, Debug)]
pub enum AppActions {
    Exit,
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
