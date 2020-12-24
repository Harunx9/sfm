use std::{fs, path::PathBuf};

use crate::app::{
    actions::{DirectoryAction, PanelSide},
    config::icon_cfg::IconsConfig,
    file_system::FileSystemItem,
    state::{AppState, PanelState, TabIdx, TabState},
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
        DirectoryAction::Open {
            path,
            tab,
            panel,
            in_new_tab,
        } => open_dir(state, path, tab, panel, in_new_tab),
        DirectoryAction::Create {
            dir_name,
            parent_path,
            tab,
            panel,
        } => create_directory(state, dir_name, parent_path, tab, panel),
    }
}

fn create_directory(
    state: AppState,
    dir_name: String,
    path: PathBuf,
    tab: TabIdx,
    panel: PanelSide,
) -> AppState {
    match panel {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: create_directory_in_tab(
                    dir_name,
                    path,
                    tab,
                    state.left_panel.tabs,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: create_directory_in_tab(
                    dir_name,
                    path,
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

fn open_dir(
    state: AppState,
    path: PathBuf,
    tab: TabIdx,
    panel: PanelSide,
    in_new_tab: bool,
) -> AppState {
    match panel {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: if in_new_tab {
                    let mut current_tabs = state.left_panel.tabs.clone();
                    current_tabs.push(TabState::with_dir(path.as_path(), &state.config.icons));

                    current_tabs
                } else {
                    open_dir_in_tab(path, tab, state.left_panel.tabs, &state.config.icons)
                },
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: if in_new_tab {
                    let mut current_tabs = state.right_panel.tabs.clone();
                    current_tabs.push(TabState::with_dir(path.as_path(), &state.config.icons));

                    current_tabs
                } else {
                    open_dir_in_tab(path, tab, state.right_panel.tabs, &state.config.icons)
                },
                ..state.right_panel
            },
            ..state
        },
    }
}

fn rename_dir(
    state: AppState,
    from: PathBuf,
    to: PathBuf,
    tab: TabIdx,
    panel: PanelSide,
) -> AppState {
    match panel {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: rename_dir_in_tab(from, to, tab, state.left_panel.tabs, &state.config.icons),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: rename_dir_in_tab(from, to, tab, state.right_panel.tabs, &state.config.icons),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn delete_dir(state: AppState, path: PathBuf, tab: TabIdx, panel: PanelSide) -> AppState {
    match panel {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: delete_dir_from_tab(path, tab, state.left_panel.tabs, &state.config.icons),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: delete_dir_from_tab(path, tab, state.right_panel.tabs, &state.config.icons),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn open_dir_in_tab(
    path: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();

    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let dir_to_open = val
                .items
                .iter()
                .find(|item| item.is_dir() && item.get_path().eq(path.as_path()));
            if let Some(item) = dir_to_open {
                result.push(TabState::with_dir(item.get_path().as_path(), icons));
            } else {
                if path.exists() {
                    result.push(TabState::with_dir(path.as_path(), icons));
                } else {
                    result.push(val.clone());
                }
            }
        } else {
            result.push(val.clone());
        }
    }

    result
}

fn rename_dir_in_tab(
    from: PathBuf,
    to: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();

    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let dir_to_rename = val
                .items
                .iter()
                .find(|item| item.is_dir() && item.get_path().eq(from.as_path()));
            if let Some(item) = dir_to_rename {
                if let FileSystemItem::Directory(dir) = item {
                    match std::fs::rename(dir.get_path(), to.clone()) {
                        Ok(_) => result.push(TabState::with_dir(val.path.as_path(), icons)),
                        Err(_) => {} //TODO: Add error handling
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

fn delete_dir_from_tab(
    path: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();

    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let dir_to_delete = val
                .items
                .iter()
                .find(|item| item.is_dir() && item.get_path().eq(path.as_path()));
            if let Some(item) = dir_to_delete {
                match std::fs::remove_dir_all(item.get_path()) {
                    Ok(_) => result.push(TabState::with_dir(val.path.as_path(), icons)),
                    Err(_) => {}
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

fn create_directory_in_tab(
    dir_name: String,
    parent_path: PathBuf,
    tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();
    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == tab {
            if parent_path.exists() {
                let dir_path = parent_path.with_extension(dir_name.clone());
                match fs::create_dir(dir_path) {
                    Ok(_) => result.push(TabState::with_dir(parent_path.as_path(), icons)),
                    Err(_) => {}
                };
            } else {
                result.push(val.clone());
            }
        } else {
            result.push(val.clone());
        }
    }
    result
}
