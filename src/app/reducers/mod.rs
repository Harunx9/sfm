use super::{
    actions::{AppAction, FileManagerActions},
    state::{AppState, PanelState},
};

mod dir_reducer;
mod file_reducer;
mod panel_reducer;
mod tab_reducer;

use dir_reducer::dir_reducer;
use file_reducer::file_reducer;
use panel_reducer::panel_reducer;
use tab_reducer::tab_reducer;

pub fn root_reducer(state: AppState, action: FileManagerActions) -> AppState {
    match action {
        FileManagerActions::App(app_action) => app_reducer(state.clone(), app_action),
        FileManagerActions::File(file_action) => file_reducer(state.clone(), file_action),
        FileManagerActions::Directory(dir_action) => dir_reducer(state.clone(), dir_action),
        FileManagerActions::Panel(panel_action) => panel_reducer(state.clone(), panel_action),
        FileManagerActions::Tab(tab_action) => tab_reducer(state.clone(), tab_action),
    }
}

fn app_reducer(state: AppState, app_action: AppAction) -> AppState {
    match app_action {
        AppAction::Exit => AppState {
            app_exit: true,
            ..state
        },
        AppAction::FocusLeft => AppState {
            left_panel: PanelState {
                is_focused: true,
                ..state.left_panel
            },
            right_panel: PanelState {
                is_focused: false,
                ..state.right_panel
            },
            ..state
        },
        AppAction::FocusRight => AppState {
            left_panel: PanelState {
                is_focused: false,
                ..state.left_panel
            },
            right_panel: PanelState {
                is_focused: true,
                ..state.right_panel
            },
            ..state
        },
    }
}
