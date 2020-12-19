use std::{fs, path::PathBuf};

use crate::app::{
    actions::{FileAction, PanelSide},
    config::IconsConfig,
    file_system::FileSystemItem,
    state::{AppState, PanelState, TabIdx, TabState},
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

fn open_file(state: AppState, path: PathBuf, tab: TabIdx, panel: PanelSide) -> AppState {
    state
}

fn delete_file(state: AppState, path: PathBuf, tab: TabIdx, panel: PanelSide) -> AppState {
    match panel {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: delete_file_from_tab(path, tab, state.left_panel.tabs, &state.config.icons),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: delete_file_from_tab(path, tab, state.right_panel.tabs, &state.config.icons),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn rename_file(
    state: AppState,
    from: PathBuf,
    to: PathBuf,
    tab: TabIdx,
    panel: PanelSide,
) -> AppState {
    match panel {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: rename_file_in_tab(from, to, tab, state.left_panel.tabs, &state.config.icons),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: rename_file_in_tab(
                    from,
                    to,
                    tab,
                    state.right_panel.tabs,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn open_file_from_tab(path: PathBuf, current_tab: TabIdx, tabs: Vec<TabState>) -> Vec<TabState> {
    tabs
}

fn delete_file_from_tab(
    path: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();

    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let item_to_delete = val
                .items
                .iter()
                .find(|item| item.is_file() && item.get_path().eq(&path));
            if let Some(item) = item_to_delete {
                if let FileSystemItem::File(file) = item {
                    match fs::remove_file(file.get_path()) {
                        Ok(_) => result.push(TabState::with_dir(val.path.as_path(), icons)),
                        Err(_) => {} //TODO: add error handling to state
                    }
                } else {
                    result.push(val.clone());
                }
            } else {
                result.push(val.clone());
            }
        } else {
            result.push(val.clone());
        }
    }

    result
}

fn rename_file_in_tab(
    from: PathBuf,
    to: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();

    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let item_to_delete = val
                .items
                .iter()
                .find(|item| item.is_file() && item.get_path().eq(&from));
            if let Some(item) = item_to_delete {
                if let FileSystemItem::File(file) = item {
                    match fs::rename(file.get_path(), to.as_path()) {
                        Ok(_) => result.push(TabState::with_dir(val.path.as_path(), icons)),
                        Err(_) => {} //TODO: add error handling to state
                    }
                } else {
                    result.push(val.clone());
                }
            } else {
                result.push(val.clone());
            }
        } else {
            result.push(val.clone());
        }
    }

    result
}
