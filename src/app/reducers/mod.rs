use super::{
    actions::{AppAction, DirectoryAction, FileAction, FileManagerActions, PanelAction},
    file_system::directory::get_items_from_dir,
    state::{AppState, PanelState},
};

mod panel_reducer;
mod tab_reducer;

use panel_reducer::panel_reducer;
use tab_reducer::tab_reducer;

pub fn root_reducer(state: AppState, action: FileManagerActions) -> AppState {
    match action {
        FileManagerActions::App(app_action) => app_reducer(state.clone(), app_action),
        FileManagerActions::File(file_action) => file_reducer(state.clone(), file_action),
        FileManagerActions::Directory(dir_action) => dir_reducer(state.clone(), dir_action),
        FileManagerActions::Panel(panel_action) => panel_reducer(state, panel_action),
        FileManagerActions::Tab(tab_action) => tab_reducer(state, tab_action),
    }
}

fn app_reducer(state: AppState, app_action: AppAction) -> AppState {
    match app_action {
        AppAction::Exit => AppState {
            left_panel: state.left_panel,
            right_panel: state.right_panel,
            config: state.config,
            app_exit: true,
        },
        AppAction::FocusLeft => AppState {
            left_panel: PanelState {
                tabs: state.left_panel.tabs,
                current_tab: state.left_panel.current_tab,
                is_focused: true,
            },
            right_panel: PanelState {
                tabs: state.right_panel.tabs,
                current_tab: state.right_panel.current_tab,
                is_focused: false,
            },
            config: state.config,
            app_exit: false,
        },
        AppAction::FocusRight => AppState {
            left_panel: PanelState {
                tabs: state.left_panel.tabs,
                current_tab: state.left_panel.current_tab,
                is_focused: false,
            },
            right_panel: PanelState {
                tabs: state.right_panel.tabs,
                current_tab: state.right_panel.current_tab,
                is_focused: true,
            },
            config: state.config,
            app_exit: false,
        },
    }
}

fn dir_reducer(state: AppState, dir_action: DirectoryAction) -> AppState {
    match dir_action {
        DirectoryAction::Delete { path } => state,
        DirectoryAction::Rename { from, to } => state,
        DirectoryAction::Move { from, to } => state,
        DirectoryAction::Open { path } => state,
        DirectoryAction::Navigate { to } => state,
    }
}

fn file_reducer(state: AppState, file_action: FileAction) -> AppState {
    match file_action {
        FileAction::Delete { path } => state,
        FileAction::Rename { from, to } => state,
        FileAction::Move { from, to } => state,
        FileAction::Open { path } => state,
    }
}
