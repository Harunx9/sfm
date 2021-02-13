use tui::widgets::ListState;

use crate::app::{
    actions::{PanelSide, SearchAction},
    file_system::FileSystem,
    state::{AppState, PanelState, TabIdx, TabState},
};
use std::fmt::Debug;

pub fn search_reducer<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    search_action: SearchAction,
) -> AppState<TFileSystem> {
    match search_action {
        SearchAction::Start { tab, panel_side } => start_search(state.clone(), tab, panel_side),
        SearchAction::Stop { tab, panel_side } => stop_search(state.clone(), tab, panel_side),
        SearchAction::Input {
            tab,
            panel_side,
            phrase,
        } => change_input(state.clone(), tab, panel_side, phrase),
        SearchAction::ApplySearch { tab, panel_side } => {
            apply_search(state.clone(), tab, panel_side)
        }
    }
}

fn apply_search<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    tab: usize,
    panel_side: PanelSide,
) -> AppState<TFileSystem> {
    match panel_side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: apply_search_in_tab(state.left_panel.tabs, tab),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: apply_search_in_tab(state.right_panel.tabs, tab),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn change_input<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    tab: usize,
    panel_side: PanelSide,
    phrase: String,
) -> AppState<TFileSystem> {
    match panel_side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: input_search_in_tab(state.left_panel.tabs, tab, phrase),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: input_search_in_tab(state.right_panel.tabs, tab, phrase),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn stop_search<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    tab: usize,
    panel_side: PanelSide,
) -> AppState<TFileSystem> {
    match panel_side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: stop_search_in_tab(state.left_panel.tabs, tab),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: stop_search_in_tab(state.right_panel.tabs, tab),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn start_search<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    tab: usize,
    panel_side: PanelSide,
) -> AppState<TFileSystem> {
    match panel_side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: start_search_in_tab(state.left_panel.tabs, tab),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: start_search_in_tab(state.right_panel.tabs, tab),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn start_search_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    tabs: Vec<TabState<TFileSystem>>,
    tab: TabIdx,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == tab {
            result.push(TabState {
                search_mode: true,
                ..tab_state.clone()
            });
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn stop_search_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    tabs: Vec<TabState<TFileSystem>>,
    tab: TabIdx,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == tab {
            result.push(TabState {
                search_mode: false,
                phrase: String::from(""),
                ..tab_state.clone()
            });
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn input_search_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    tabs: Vec<TabState<TFileSystem>>,
    tab: TabIdx,
    phrase: String,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == tab && tab_state.search_mode {
            result.push(TabState {
                phrase: phrase.clone(),
                ..tab_state.clone()
            });
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}

fn apply_search_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    tabs: Vec<TabState<TFileSystem>>,
    tab: usize,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();

    for (idx, tab_state) in tabs.iter().enumerate() {
        if idx == tab {
            result.push(TabState {
                search_mode: false,
                tab_state: ListState::default(),
                ..tab_state.clone()
            });
        } else {
            result.push(tab_state.clone());
        }
    }

    result
}
