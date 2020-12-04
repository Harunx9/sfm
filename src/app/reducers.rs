use super::{
    actions::{AppAction, DirectoryAction, FileAction, FileManagerActions},
    file_system::directory::get_items_from_dir,
    state::{AppState, TabState},
};

pub fn root_reducer(state: AppState, action: FileManagerActions) -> AppState {
    match action {
        FileManagerActions::App(app_action) => app_reducer(state.clone(), app_action),
        FileManagerActions::File(file_action) => file_reducer(state.clone(), file_action),
        FileManagerActions::Directory(dir_action) => dir_reducer(state.clone(), dir_action),
    }
}

fn app_reducer(state: AppState, app_action: AppAction) -> AppState {
    match app_action {
        AppAction::Exit => AppState {
            left_tab: state.left_tab,
            right_tab: state.right_tab,
            app_exit: true,
        },
        AppAction::FocusLeft => AppState {
            left_tab: TabState {
                name: state.left_tab.name,
                path: state.left_tab.path,
                items: state.left_tab.items,
                is_focused: true,
            },
            right_tab: TabState {
                name: state.right_tab.name,
                path: state.right_tab.path,
                items: state.right_tab.items,
                is_focused: false,
            },
            app_exit: false,
        },
        AppAction::FocusRight => AppState {
            left_tab: TabState {
                name: state.left_tab.name,
                path: state.left_tab.path,
                items: state.left_tab.items,
                is_focused: false,
            },
            right_tab: TabState {
                name: state.right_tab.name,
                path: state.right_tab.path,
                items: state.right_tab.items,
                is_focused: true,
            },
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
