#[derive(Clone, Debug)]
pub enum FrActions {
    App(AppActions),
}

#[derive(Clone, Debug)]
pub enum AppActions {
    Exit,
}
