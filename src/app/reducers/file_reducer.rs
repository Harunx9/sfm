use std::fmt::Debug;
use std::{ffi::OsStr, path::PathBuf};

use crate::app::{
    actions::{FileAction, PanelInfo, PanelSide},
    config::{icon_cfg::IconsConfig, program_associations::FileAssociatedPrograms},
    file_system::{file_system_item::FileSystemItem, FileSystem},
    state::{AppState, ChildProgramDesc, PanelState, TabIdx, TabState},
};

use super::{reload_tab, reload_tab_contain_item, reload_tab_with_path};

pub fn file_reducer<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    file_action: FileAction,
) -> AppState<TFileSystem> {
    match file_action {
        FileAction::Delete { panel } => delete_file(state, panel),
        FileAction::Rename { from, to } => rename_file(state, from, to),
        FileAction::Move { from, to } => rename_file(state, from, to),
        FileAction::Open { panel } => open_file(state, panel),
        FileAction::Create { file_name, panel } => create_file(state, file_name, panel),
        FileAction::Copy { from, to } => copy_file(state, from, to),
    }
}

fn copy_file<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    from: PanelInfo,
    to: PanelInfo,
) -> AppState<TFileSystem> {
    match to.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: copy_file_to_tab(
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
                tabs: copy_file_to_tab(
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

fn create_file<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    file_name: String,
    panel: PanelInfo,
) -> AppState<TFileSystem> {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: create_file_in_tab(
                    file_name,
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
                tabs: create_file_in_tab(
                    file_name,
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

fn open_file<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    panel: PanelInfo,
) -> AppState<TFileSystem> {
    AppState {
        child_program: open_file_from_tab(panel.path, &state.config.file_associated_programs),
        ..state
    }
}

fn delete_file<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    panel: PanelInfo,
) -> AppState<TFileSystem> {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: delete_file_from_tab(
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
                tabs: delete_file_from_tab(
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

fn rename_file<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    from: PanelInfo,
    to: PanelInfo,
) -> AppState<TFileSystem> {
    match to.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: rename_file_in_tab(
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
                tabs: rename_file_in_tab(
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
                ..state.right_panel
            },
            ..state
        },
    }
}

fn create_file_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    file_name: String,
    dir_path: PathBuf,
    tab: TabIdx,
    mut tabs: Vec<TabState<TFileSystem>>,
    file_system: &mut TFileSystem,
    icons: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for (idx, tab_state) in tabs.iter_mut().enumerate() {
        if idx == tab {
            if dir_path.exists() {
                let mut file_path = dir_path.clone();
                file_path.push(file_name.clone());
                match file_system.create_file(&file_path) {
                    Ok(_) => {
                        result.push(TabState::with_dir(dir_path.as_path(), file_system, icons))
                    }
                    Err(_) => {}
                }
            } else {
                result.push(tab_state.clone());
            }
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn open_file_from_tab(
    path: PathBuf,
    file_associated_programs: &FileAssociatedPrograms,
) -> Option<ChildProgramDesc> {
    if path.is_file() && path.exists() {
        let file_extension = path.extension().unwrap_or(OsStr::new(""));
        Some(ChildProgramDesc {
            program_name: file_associated_programs
                .get_program_name(String::from(file_extension.to_str().unwrap())),
            args: vec![String::from(path.to_str().unwrap())],
        })
    } else {
        None
    }
}

fn delete_file_from_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    path: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState<TFileSystem>>,
    file_system: &mut TFileSystem,
    icons: &IconsConfig,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, tab_state) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let item_to_delete = tab_state
                .items
                .iter()
                .find(|item| item.is_file() && item.get_path().eq(&path));
            if let Some(item) = item_to_delete {
                if let FileSystemItem::File(file) = item {
                    match file_system.delete_file(&file.get_path()) {
                        Ok(_) => result.push(TabState::with_dir(
                            tab_state.path.as_path(),
                            file_system,
                            icons,
                        )),
                        Err(_) => {} //TODO: add error handling to state
                    }
                } else {
                    result.push(tab_state.clone());
                }
            } else {
                result.push(tab_state.clone());
            }
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn copy_file_to_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
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
            match file_system.copy_file(from.as_path(), to.as_path()) {
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

fn rename_file_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
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
            match file_system.rename_item(&from.as_path(), &to.as_path()) {
                Ok(_) => result.push(TabState::with_dir(
                    tab_state.path.as_path(),
                    file_system,
                    icons,
                )),
                Err(_) => {} //TODO: add error handling to state
            }
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}
