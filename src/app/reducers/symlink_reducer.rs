use std::{fmt::Debug, path::PathBuf};

use crate::app::{
    actions::{PanelSide, SymlinkAction},
    file_system::FileSystem,
    state::{AppState, PanelState, TabIdx, TabState},
};

pub fn symlink_reducer<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    action: SymlinkAction,
) -> AppState<TFileSystem> {
    match action {
        SymlinkAction::Create {
            symlink_path,
            panel,
        } => create_symlink(state, symlink_path, panel),
        _ => state,
    }
}

fn create_symlink<TFileSystem: Clone + Debug + Default + FileSystem>(
    mut state: AppState<TFileSystem>,
    symlink_path: PathBuf,
    panel: crate::app::actions::PanelInfo,
) -> AppState<TFileSystem> {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: create_symlink_in_tab(
                    symlink_path,
                    panel.tab,
                    panel.path,
                    &mut state.file_system,
                    state.left_panel.tabs,
                ),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: create_symlink_in_tab(
                    symlink_path,
                    panel.tab,
                    panel.path,
                    &mut state.file_system,
                    state.right_panel.tabs,
                ),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn create_symlink_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    symlink_path: PathBuf,
    tab: TabIdx,
    path: PathBuf,
    file_system: &mut TFileSystem,
    tabs: Vec<TabState<TFileSystem>>,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == tab {
            match file_system.create_symlink(&symlink_path, &path) {
                Ok(_) => result.push(tab_state.clone()),
                Err(err) => {
                    eprintln!("{}", err);
                    result.push(tab_state.clone())
                }
            }
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}
