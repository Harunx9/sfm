use super::{
    actions::{AppActions, FrActions},
    state::State,
};

pub fn root_reducer(state: State, action: FrActions) -> State {
    match action {
        FrActions::App(app) => app_reducer(state.clone(), app),
    }
}

pub fn app_reducer(state: State, app_actions: AppActions) -> State {
    match app_actions {
        AppActions::Exit => State {
            left_tab: state.left_tab,
            right_tab: state.right_tab,
            app_exit: true,
        },
    }
}
