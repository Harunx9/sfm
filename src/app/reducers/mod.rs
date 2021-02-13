use super::{
    actions::{AppAction, FileManagerActions},
    config::icon_cfg::IconsConfig,
    file_system::FileSystem,
    state::{AppState, PanelState, TabIdx, TabState},
};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

mod dir_reducer;
mod file_reducer;
mod panel_reducer;
mod search_reducer;
mod symlink_reducer;
mod tab_reducer;

use dir_reducer::dir_reducer;
use file_reducer::file_reducer;
use panel_reducer::panel_reducer;
use search_reducer::search_reducer;
use symlink_reducer::symlink_reducer;
use tab_reducer::tab_reducer;

pub fn root_reducer<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    action: FileManagerActions,
) -> AppState<TFileSystem> {
    match action {
        FileManagerActions::App(app_action) => app_reducer(state.clone(), app_action),
        FileManagerActions::File(file_action) => file_reducer(state.clone(), file_action),
        FileManagerActions::Directory(dir_action) => dir_reducer(state.clone(), dir_action),
        FileManagerActions::Symlink(symlink_action) => {
            symlink_reducer(state.clone(), symlink_action)
        }
        FileManagerActions::Panel(panel_action) => panel_reducer(state.clone(), panel_action),
        FileManagerActions::Tab(tab_action) => tab_reducer(state.clone(), tab_action),
        FileManagerActions::Search(search_action) => search_reducer(state.clone(), search_action),
    }
}

fn app_reducer<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    app_action: AppAction,
) -> AppState<TFileSystem> {
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
        AppAction::ChildProgramClosed => AppState {
            child_program: None,
            ..state
        },
        AppAction::ShowModal(modal_type) => AppState {
            modal: Some(modal_type),
            ..state
        },
        AppAction::CloseModal => AppState {
            modal: None,
            ..state
        },
    }
}

fn reload_tab<TFileSystem: Clone + Default + Debug + FileSystem>(
    tab: TabIdx,
    tabs: Vec<TabState<TFileSystem>>,
    file_system: &mut TFileSystem,
    icons_cfg: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == tab {
            result.push(TabState::with_dir(
                tab_state.path.as_path(),
                file_system,
                icons_cfg,
            ));
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn reload_tab_with_path<TFileSystem: Clone + Default + Debug + FileSystem>(
    tab_path: &Path,
    tabs: Vec<TabState<TFileSystem>>,
    file_system: &TFileSystem,
    icons_cfg: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for tab_state in tabs.iter() {
        if tab_state.path == tab_path {
            result.push(TabState::with_dir(
                tab_state.path.as_path(),
                file_system,
                icons_cfg,
            ));
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn reload_tab_contain_item<TFileSystem: Clone + Default + Debug + FileSystem>(
    path: PathBuf,
    tabs: Vec<TabState<TFileSystem>>,
    file_system: &TFileSystem,
    icons_cfg: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for tab_state in tabs.iter() {
        result.push(reload_if_contain(
            tab_state,
            path.clone(),
            file_system,
            icons_cfg,
        ));
    }

    result
}

fn reload_if_contain<TFileSystem: Clone + Default + Debug + FileSystem>(
    tab_state: &TabState<TFileSystem>,
    path: PathBuf,
    file_system: &TFileSystem,
    icons_cfg: &IconsConfig,
) -> TabState<TFileSystem> {
    if tab_state.items.iter().any(|i| i.get_path() == path) {
        TabState::with_dir(tab_state.path.as_path(), file_system, icons_cfg)
    } else {
        tab_state.clone()
    }
}
