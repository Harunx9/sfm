use std::{
    ffi::OsStr,
    fs::{self, File},
    path::PathBuf,
};

use crate::app::{
    actions::{FileAction, PanelInfo, PanelSide},
    config::{icon_cfg::IconsConfig, program_associations::FileAssociatedPrograms},
    file_system::FileSystemItem,
    state::{AppState, ChildProgramDesc, PanelState, TabIdx, TabState},
};

use super::reload_tab;

pub fn file_reducer(state: AppState, file_action: FileAction) -> AppState {
    match file_action {
        FileAction::Delete { panel } => delete_file(state, panel),
        FileAction::Rename { from, to } => rename_file(state, from, to),
        FileAction::Move { from, to } => rename_file(state, from, to),
        FileAction::Open { panel } => open_file(state, panel),
        FileAction::Create { file_name, panel } => create_file(state, file_name, panel),
    }
}

fn create_file(state: AppState, file_name: String, panel: PanelInfo) -> AppState {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: create_file_in_tab(
                    file_name,
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
                tabs: create_file_in_tab(
                    file_name,
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

fn open_file(state: AppState, panel: PanelInfo) -> AppState {
    AppState {
        child_program: open_file_from_tab(panel.path, &state.config.file_associated_programs),
        ..state
    }
}

fn delete_file(state: AppState, panel: PanelInfo) -> AppState {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: delete_file_from_tab(
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
                tabs: delete_file_from_tab(
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

fn rename_file(state: AppState, from: PanelInfo, to: PanelInfo) -> AppState {
    match to.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: rename_file_in_tab(
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
            right_panel: PanelState {
                tabs: rename_file_in_tab(
                    from.path,
                    to.path,
                    to.tab,
                    state.right_panel.tabs,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            left_panel: PanelState {
                tabs: reload_tab(from.tab, state.left_panel.tabs, &state.config.icons),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn create_file_in_tab(
    file_name: String,
    dir_path: PathBuf,
    tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();
    for (idx, tab_state) in tabs.iter_mut().enumerate() {
        if idx == tab {
            if dir_path.exists() {
                let mut file_path = dir_path.clone();
                file_path.push(file_name.clone());
                match File::create(file_path) {
                    Ok(_) => result.push(TabState::with_dir(dir_path.as_path(), icons)),
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

fn delete_file_from_tab(
    path: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();

    for (idx, tab_state) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            let item_to_delete = tab_state
                .items
                .iter()
                .find(|item| item.is_file() && item.get_path().eq(&path));
            if let Some(item) = item_to_delete {
                if let FileSystemItem::File(file) = item {
                    match fs::remove_file(file.get_path()) {
                        Ok(_) => result.push(TabState::with_dir(tab_state.path.as_path(), icons)),
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

fn rename_file_in_tab(
    from: PathBuf,
    to: PathBuf,
    current_tab: TabIdx,
    mut tabs: Vec<TabState>,
    icons: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();
    for (idx, tab_state) in tabs.iter_mut().enumerate() {
        if idx == current_tab {
            match fs::rename(from.as_path(), to.as_path()) {
                Ok(_) => result.push(TabState::with_dir(tab_state.path.as_path(), icons)),
                Err(_) => {} //TODO: add error handling to state
            }
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}
