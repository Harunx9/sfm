use super::{
    actions::{AppActions, FrActions},
    state::AppState,
};

pub fn root_reducer(state: AppState, action: FrActions) -> AppState {
    match action {
        FrActions::App(app) => app_reducer(state.clone(), app),
        FrActions::File(_) => state,
        FrActions::Directory(_) => state,
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
