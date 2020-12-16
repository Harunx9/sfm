use crate::app::{
    actions::{FileAction, PanelSide},
    state::AppState,
};

pub fn file_reducer(state: AppState, file_action: FileAction) -> AppState {
    match file_action {
        FileAction::Delete { path, tab, panel } => delete_file(state, path, tab, panel),
        FileAction::Rename {
            from,
            to,
            tab,
            panel,
        } => rename_file(state, from, to, tab, panel),
        FileAction::Move {
            from,
            to,
            tab,
            panel,
        } => rename_file(state, from, to, tab, panel),
        FileAction::Open { path, tab, panel } => open_file(state, path, tab, panel),
    }
}

fn open_file(state: AppState, path: String, tab: String, panel: PanelSide) -> AppState {
    todo!()
}

fn delete_file(state: AppState, path: String, tab: String, panel: PanelSide) -> AppState {
    state
}

fn rename_file(
    state: AppState,
    form: String,
    to: String,
    tab: String,
    panel: PanelSide,
) -> AppState {
    state
}
