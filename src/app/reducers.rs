use super::{
    actions::{AppActions, FileManagerActions},
    state::AppState,
};

pub fn root_reducer(state: AppState, action: FileManagerActions) -> AppState {
    match action {
        FileManagerActions::App(app) => app_reducer(state.clone(), app),
        FileManagerActions::File(_) => state,
        FileManagerActions::Directory(_) => state,
    }
}

pub fn app_reducer(state: AppState, app_actions: AppActions) -> AppState {
    match app_actions {
        AppActions::Exit => AppState {
            left_tab: state.left_tab,
            right_tab: state.right_tab,
            app_exit: true,
        },
    }
}
