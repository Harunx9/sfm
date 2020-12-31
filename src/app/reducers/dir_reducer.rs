use std::{fs, path::PathBuf};

use crate::app::{
    actions::{DirectoryAction, PanelInfo, PanelSide},
    config::icon_cfg::IconsConfig,
    state::{AppState, PanelState, TabIdx, TabState},
};

use super::reload_tab;

pub fn dir_reducer(state: AppState, dir_action: DirectoryAction) -> AppState {
    match dir_action {
        DirectoryAction::Delete { panel } => delete_dir(state, panel),
        DirectoryAction::Rename { from, to } => rename_dir(state, from, to),
        DirectoryAction::Move { from, to } => rename_dir(state, from, to),
        DirectoryAction::Open { panel, in_new_tab } => open_dir(state, panel, in_new_tab),
        DirectoryAction::Create { dir_name, panel } => create_directory(state, dir_name, panel),
    }
}

fn create_directory(state: AppState, dir_name: String, panel: PanelInfo) -> AppState {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: create_directory_in_tab(
                    dir_name,
                    panel.path,
                    panel.tab,
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
                    panel.path,
                    panel.tab,
                    state.right_panel.tabs,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn open_dir(state: AppState, panel: PanelInfo, in_new_tab: bool) -> AppState {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: if in_new_tab {
                    let mut current_tabs = state.left_panel.tabs.clone();
                    current_tabs.push(TabState::with_dir(
                        panel.path.as_path(),
                        &state.config.icons,
                    ));

                    current_tabs
                } else {
                    open_dir_in_tab(
                        panel.path,
                        panel.tab,
                        state.left_panel.tabs,
                        &state.config.icons,
                    )
                },
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: if in_new_tab {
                    let mut current_tabs = state.right_panel.tabs.clone();
                    current_tabs.push(TabState::with_dir(
                        panel.path.as_path(),
                        &state.config.icons,
                    ));

                    current_tabs
                } else {
                    open_dir_in_tab(
                        panel.path,
                        panel.tab,
                        state.right_panel.tabs,
                        &state.config.icons,
                    )
                },
                ..state.right_panel
            },
            ..state
        },
    }
}

fn rename_dir(state: AppState, from: PanelInfo, to: PanelInfo) -> AppState {
    match to.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: rename_dir_in_tab(
                    from.path,
                    to.path,
                    to.tab,
                    state.left_panel.tabs,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            right_panel: PanelState {
                tabs: reload_tab(from.tab, state.right_panel.tabs, &state.config.icons),
                ..state.right_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            left_panel: PanelState {
                tabs: reload_tab(from.tab, state.left_panel.tabs, &state.config.icons),
                ..state.left_panel
            },
            right_panel: PanelState {
                tabs: rename_dir_in_tab(
                    from.path,
                    to.path,
                    to.tab,
                    state.right_panel.tabs,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn delete_dir(state: AppState, panel: PanelInfo) -> AppState {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: delete_dir_from_tab(
                    panel.path,
                    panel.tab,
                    state.left_panel.tabs,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: delete_dir_from_tab(
                    panel.path,
                    panel.tab,
                    state.right_panel.tabs,
                    &state.config.icons,
                ),
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
    tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();

    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == current_tab {
            match std::fs::rename(from.clone(), to.clone()) {
                Ok(_) => result.push(TabState::with_dir(tab_state.path.as_path(), icons)),
                Err(_) => {} //TODO: Add error handling
            }
        } else {
            result.push(tab_state.clone());
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
                let mut dir_path = parent_path.clone();
                dir_path.push(dir_name.clone());

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
