use crate::app::{
    actions::{DirectoryAction, PanelSide},
    state::AppState,
};

pub fn dir_reducer(state: AppState, dir_action: DirectoryAction) -> AppState {
    match dir_action {
        DirectoryAction::Delete { path, tab, panel } => delete_dir(state, path, tab, panel),
        DirectoryAction::Rename {
            from,
            to,
            tab,
            panel,
        } => rename_dir(state, from, to, tab, panel),
        DirectoryAction::Move {
            from,
            to,
            tab,
            panel,
        } => rename_dir(state, from, to, tab, panel),
        DirectoryAction::Open { path, tab, panel } => open_dir(state, path, tab, panel),
    }
}

fn open_dir(state: AppState, path: String, tab: String, panel: PanelSide) -> AppState {
    todo!()
}

fn rename_dir(
    state: AppState,
    from: String,
    to: String,
    tab: String,
    panel: PanelSide,
) -> AppState {
    todo!()
}

fn delete_dir(state: AppState, path: String, tab: String, panel: PanelSide) -> AppState {
    todo!()
}
