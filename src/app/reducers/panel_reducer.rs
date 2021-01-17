use crate::app::{
    actions::{PanelAction, PanelSide},
    file_system::FileSystem,
    state::{AppState, PanelState, TabIdx},
};
use std::fmt::Debug;

pub fn panel_reducer<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    panel_action: PanelAction,
) -> AppState<TFileSystem> {
    match panel_action {
        PanelAction::Next { panel } => next_tab(state, panel),
        PanelAction::Previous { panel } => prev_tab(state, panel),
        PanelAction::CloseTab { tab, panel } => close_tab(state, tab, panel),
    }
}

fn close_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    tab: TabIdx,
    panel: PanelSide,
) -> AppState<TFileSystem> {
    match panel {
        PanelSide::Left => AppState {
            left_panel: close_tab_in_panel(state.left_panel, tab),
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: close_tab_in_panel(state.right_panel, tab),
            ..state
        },
    }
}

fn close_tab_in_panel<TFileSystem: Clone + Debug + Default + FileSystem>(
    panel_state: PanelState<TFileSystem>,
    tab: TabIdx,
) -> PanelState<TFileSystem> {
    let tabs: Vec<_> = panel_state
        .tabs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != tab)
        .map(|(_, v)| v.clone())
        .collect();

    let tabs_len = tabs.len();
    PanelState {
        tabs,
        current_tab: if panel_state.current_tab > tab {
            panel_state.current_tab - 1
        } else if tab >= tabs_len {
            tabs_len - 1
        } else if panel_state.current_tab == 0 {
            0
        } else {
            panel_state.current_tab
        },
        is_focused: panel_state.is_focused,
        marker: std::marker::PhantomData,
    }
}

fn prev_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    panel: PanelSide,
) -> AppState<TFileSystem> {
    match panel {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                current_tab: if state.left_panel.current_tab == 0 {
                    state.left_panel.tabs.len() - 1
                } else {
                    state.left_panel.current_tab - 1
                },
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                current_tab: if state.right_panel.current_tab == 0 {
                    state.right_panel.tabs.len() - 1
                } else {
                    state.right_panel.current_tab - 1
                },
                ..state.right_panel
            },
            ..state
        },
    }
}

fn next_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    panel: PanelSide,
) -> AppState<TFileSystem> {
    match panel {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                current_tab: if state.left_panel.current_tab >= state.left_panel.tabs.len() - 1 {
                    0
                } else {
                    state.left_panel.current_tab + 1
                },
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                current_tab: if state.right_panel.current_tab >= state.right_panel.tabs.len() - 1 {
                    0
                } else {
                    state.right_panel.current_tab + 1
                },
                ..state.right_panel
            },
            ..state
        },
    }
}
