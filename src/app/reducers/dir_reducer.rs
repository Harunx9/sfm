use crate::app::{
    actions::{DirectoryAction, PanelInfo, PanelSide},
    config::icon_cfg::IconsConfig,
    file_system::FileSystem,
    state::{AppState, PanelState, TabIdx, TabState},
};
use std::fmt::Debug;
use std::path::PathBuf;

use super::{reload_tab, reload_tab_contain_item, reload_tab_with_path};

pub fn dir_reducer<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    dir_action: DirectoryAction,
) -> AppState<TFileSystem> {
    match dir_action {
        DirectoryAction::Delete { panel, .. } => delete_dir(state, panel),
        DirectoryAction::Rename { from, to } => rename_dir(state, from, to),
        DirectoryAction::Move { from, to } => rename_dir(state, from, to),
        DirectoryAction::Open { panel, in_new_tab } => open_dir(state, panel, in_new_tab),
        DirectoryAction::Create { dir_name, panel } => create_directory(state, dir_name, panel),
        DirectoryAction::DeleteWithContent { panel } => delete_dir_with_content(state, panel),
        DirectoryAction::Copy { from, to } => copy_dir(state, from, to),
    }
}

fn copy_dir<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    from: PanelInfo,
    to: PanelInfo,
) -> AppState<TFileSystem> {
    match to.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: copy_dir_to_tab(
                    from.path,
                    to.path,
                    to.tab,
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            right_panel: PanelState {
                tabs: reload_tab(
                    from.tab,
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: copy_dir_to_tab(
                    from.path,
                    to.path,
                    to.tab,
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            left_panel: PanelState {
                tabs: reload_tab(
                    from.tab,
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            ..state
        },
    }
}

fn delete_dir_with_content<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    panel: PanelInfo,
) -> AppState<TFileSystem> {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: delete_dir_with_content_from_tab(
                    panel.path.clone(),
                    panel.tab,
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            right_panel: PanelState {
                tabs: reload_tab_contain_item(
                    panel.path.clone(),
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: delete_dir_with_content_from_tab(
                    panel.path.clone(),
                    panel.tab,
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            left_panel: PanelState {
                tabs: reload_tab_contain_item(
                    panel.path.clone(),
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            ..state
        },
    }
}

fn create_directory<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    dir_name: String,
    panel: PanelInfo,
) -> AppState<TFileSystem> {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: create_directory_in_tab(
                    dir_name,
                    panel.path.clone(),
                    panel.tab,
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            right_panel: PanelState {
                tabs: reload_tab_with_path(
                    panel.path.as_path(),
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: create_directory_in_tab(
                    dir_name,
                    panel.path.clone(),
                    panel.tab,
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            left_panel: PanelState {
                tabs: reload_tab_with_path(
                    panel.path.as_path(),
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            ..state
        },
    }
}

fn open_dir<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    panel: PanelInfo,
    in_new_tab: bool,
) -> AppState<TFileSystem> {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: if in_new_tab {
                    let mut current_tabs = state.left_panel.tabs.clone();
                    current_tabs.push(TabState::with_dir(
                        panel.path.as_path(),
                        &state.file_system,
                        &state.config.icons,
                    ));

                    current_tabs
                } else {
                    open_dir_in_tab(
                        panel.path,
                        panel.tab,
                        state.left_panel.tabs,
                        &state.file_system,
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
                        &state.file_system,
                        &state.config.icons,
                    ));

                    current_tabs
                } else {
                    open_dir_in_tab(
                        panel.path,
                        panel.tab,
                        state.right_panel.tabs,
                        &state.file_system,
                        &state.config.icons,
                    )
                },
                ..state.right_panel
            },
            ..state
        },
    }
}

fn rename_dir<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    from: PanelInfo,
    to: PanelInfo,
) -> AppState<TFileSystem> {
    match to.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: rename_dir_in_tab(
                    from.path,
                    to.path,
                    to.tab,
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            right_panel: PanelState {
                tabs: reload_tab(
                    from.tab,
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: rename_dir_in_tab(
                    from.path,
                    to.path,
                    to.tab,
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            left_panel: PanelState {
                tabs: reload_tab(
                    from.tab,
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            ..state
        },
    }
}

fn delete_dir<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    panel: PanelInfo,
) -> AppState<TFileSystem> {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: delete_dir_from_tab(
                    panel.path.clone(),
                    panel.tab,
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            right_panel: PanelState {
                tabs: reload_tab_contain_item(
                    panel.path.clone(),
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: delete_dir_from_tab(
                    panel.path.clone(),
                    panel.tab,
                    state.right_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            left_panel: PanelState {
                tabs: reload_tab_contain_item(
                    panel.path.clone(),
                    state.left_panel.tabs,
                    &mut state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            ..state
        },
    }
}

fn copy_dir_to_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    from: PathBuf,
    to: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState<TFileSystem>>,
    file_system: &mut TFileSystem,
    icons: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, tab_state) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            match file_system.copy_dir(from.as_path(), to.as_path()) {
                Ok(_) => result.push(TabState::with_dir(
                    tab_state.path.as_path(),
                    file_system,
                    icons,
                )),
                Err(_) => {}
            }
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn open_dir_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    path: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState<TFileSystem>>,
    file_system: &TFileSystem,
    icons: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let dir_to_open = val
                .items
                .iter()
                .find(|item| item.is_dir() && item.get_path().eq(path.as_path()));
            if let Some(item) = dir_to_open {
                result.push(TabState::with_dir(
                    item.get_path().as_path(),
                    file_system,
                    icons,
                ));
            } else {
                if path.exists() {
                    result.push(TabState::with_dir(path.as_path(), file_system, icons));
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

fn rename_dir_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    from: PathBuf,
    to: PathBuf,
    current_tab: TabIdx,
    tabs: Vec<TabState<TFileSystem>>,
    file_system: &mut TFileSystem,
    icons: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == current_tab {
            match file_system.rename_item(&from, &to) {
                Ok(_) => result.push(TabState::with_dir(
                    tab_state.path.as_path(),
                    file_system,
                    icons,
                )),
                Err(_) => result.push(TabState::with_dir(
                    //TODO: temporary fix add proper error handling in reducers
                    tab_state.path.as_path(),
                    file_system,
                    icons,
                )),
            }
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn delete_dir_from_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    path: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState<TFileSystem>>,
    file_system: &mut TFileSystem,
    icons: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let dir_to_delete = val
                .items
                .iter()
                .find(|item| item.is_dir() && item.get_path().eq(path.as_path()));
            if let Some(item) = dir_to_delete {
                match file_system.delete_empty_dir(&item.get_path()) {
                    Ok(_) => {
                        result.push(TabState::with_dir(val.path.as_path(), file_system, icons))
                    }
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

fn delete_dir_with_content_from_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    path: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState<TFileSystem>>,
    file_system: &mut TFileSystem,
    icons: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let dir_to_delete = val
                .items
                .iter()
                .find(|item| item.is_dir() && item.get_path().eq(path.as_path()));
            if let Some(item) = dir_to_delete {
                match file_system.delete_dir(&item.get_path()) {
                    Ok(_) => {
                        result.push(TabState::with_dir(val.path.as_path(), file_system, icons))
                    }
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
fn create_directory_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    dir_name: String,
    parent_path: PathBuf,
    tab: TabIdx,
    mut tabs: Vec<TabState<TFileSystem>>,
    file_system: &mut TFileSystem,
    icons: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for (idx, val) in tabs.iter_mut().enumerate() {
        if idx == tab {
            if parent_path.exists() {
                let mut dir_path = parent_path.clone();
                dir_path.push(dir_name.clone());

                match file_system.create_dir(&dir_path) {
                    Ok(_) => result.push(TabState::with_dir(
                        parent_path.as_path(),
                        file_system,
                        icons,
                    )),
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
