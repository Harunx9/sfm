use std::{io, path::PathBuf};

use crate::app::{
    actions::{PanelSide, SymlinkAction},
    config::icon_cfg::IconsConfig,
    file_system::path::expand_if_contains_tilde,
    state::{AppState, PanelState, TabIdx, TabState},
};

#[cfg(unix)]
use std::os::unix::fs;
#[cfg(windows)]
use std::os::windows::fs;

pub fn symlink_reducer(state: AppState, action: SymlinkAction) -> AppState {
    match action {
        SymlinkAction::Create {
            symlink_path,
            panel,
        } => create_symlink(state, symlink_path, panel),
        _ => state,
    }
}

fn create_symlink(
    state: AppState,
    symlink_path: PathBuf,
    panel: crate::app::actions::PanelInfo,
) -> AppState {
    match panel.side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: create_symlink_in_tab(
                    symlink_path,
                    panel.tab,
                    panel.path,
                    state.left_panel.tabs,
                    &state.config.icons,
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
                    state.right_panel.tabs,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
    }
}

#[cfg(unix)]
fn create_link(symlink_path: PathBuf, item_path: PathBuf) -> io::Result<()> {
    let symlink_path = expand_if_contains_tilde(symlink_path.as_path()).unwrap();
    fs::symlink(item_path.as_path(), symlink_path.as_path())
}

#[cfg(windows)]
fn create_link(symlink_path: PathBuf, item_path: PathBuf) -> io::Result<()> {}

fn create_symlink_in_tab(
    symlink_path: PathBuf,
    tab: TabIdx,
    path: PathBuf,
    tabs: Vec<TabState>,
    icons_config: &IconsConfig,
) -> Vec<TabState> {
    let mut result = Vec::<TabState>::new();

    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == tab {
            match create_link(symlink_path.clone(), path.clone()) {
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
