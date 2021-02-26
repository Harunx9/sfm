use crate::app::{
    actions::{PanelSide, TabAction},
    file_system::FileSystem,
    state::{AppState, PanelState, TabState},
};
use std::{fmt::Debug, path::PathBuf};

use super::reload_tab_with_path;

pub fn tab_reducer<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    tab_action: TabAction,
) -> AppState<TFileSystem> {
    match tab_action {
        TabAction::Next => select_next(state),
        TabAction::Previous => select_previous(state),
        TabAction::SelectNext => select_multiple_next(state),
        TabAction::SelectPrev => select_multiple_prev(state),
        TabAction::ClearSelection => clear_selections(state),
        TabAction::ReloadTab { panel_side, path } => reload_state_tab(state, panel_side, path),
    }
}

fn reload_state_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
    panel_side: PanelSide,
    path: PathBuf,
) -> AppState<TFileSystem> {
    match panel_side {
        PanelSide::Left => AppState {
            left_panel: PanelState {
                tabs: reload_tab_with_path(
                    path.as_ref(),
                    state.left_panel.tabs,
                    &state.file_system,
                    &state.config.icons,
                ),
                ..state.left_panel
            },
            ..state
        },
        PanelSide::Right => AppState {
            right_panel: PanelState {
                tabs: reload_tab_with_path(
                    path.as_ref(),
                    state.right_panel.tabs,
                    &state.file_system,
                    &state.config.icons,
                ),
                ..state.right_panel
            },
            ..state
        },
    }
}

fn clear_selections<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
) -> AppState<TFileSystem> {
    if state.left_panel.is_focused {
        AppState {
            left_panel: PanelState {
                tabs: clear_selections_in_tab(state.left_panel.current_tab, state.left_panel.tabs),
                ..state.left_panel
            },
            ..state
        }
    } else if state.right_panel.is_focused {
        AppState {
            right_panel: PanelState {
                tabs: clear_selections_in_tab(
                    state.right_panel.current_tab,
                    state.right_panel.tabs,
                ),
                ..state.right_panel
            },
            ..state
        }
    } else {
        AppState { ..state }
    }
}

fn select_multiple_prev<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
) -> AppState<TFileSystem> {
    if state.left_panel.is_focused {
        AppState {
            left_panel: PanelState {
                tabs: select_multiple_prev_in_tab(
                    state.left_panel.current_tab,
                    state.left_panel.tabs,
                ),
                ..state.left_panel
            },
            ..state
        }
    } else if state.right_panel.is_focused {
        AppState {
            right_panel: PanelState {
                tabs: select_multiple_prev_in_tab(
                    state.right_panel.current_tab,
                    state.right_panel.tabs,
                ),
                ..state.right_panel
            },
            ..state
        }
    } else {
        AppState { ..state }
    }
}

fn select_multiple_next<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
) -> AppState<TFileSystem> {
    if state.left_panel.is_focused {
        AppState {
            left_panel: PanelState {
                tabs: select_multiple_next_in_tab(
                    state.left_panel.current_tab,
                    state.left_panel.tabs,
                ),
                ..state.left_panel
            },
            ..state
        }
    } else if state.right_panel.is_focused {
        AppState {
            right_panel: PanelState {
                tabs: select_multiple_next_in_tab(
                    state.right_panel.current_tab,
                    state.right_panel.tabs,
                ),
                ..state.right_panel
            },
            ..state
        }
    } else {
        AppState { ..state }
    }
}

fn select_next<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
) -> AppState<TFileSystem> {
    if state.left_panel.is_focused {
        AppState {
            left_panel: PanelState {
                tabs: select_next_element(state.left_panel.current_tab, state.left_panel.tabs),
                ..state.left_panel
            },
            ..state
        }
    } else if state.right_panel.is_focused {
        AppState {
            right_panel: PanelState {
                tabs: select_next_element(state.right_panel.current_tab, state.right_panel.tabs),
                ..state.right_panel
            },
            ..state
        }
    } else {
        AppState { ..state }
    }
}

fn select_previous<TFileSystem: Clone + Debug + Default + FileSystem>(
    state: AppState<TFileSystem>,
) -> AppState<TFileSystem> {
    if state.left_panel.is_focused {
        AppState {
            left_panel: PanelState {
                tabs: select_prev_element(state.left_panel.current_tab, state.left_panel.tabs),
                ..state.left_panel
            },
            ..state
        }
    } else if state.right_panel.is_focused {
        AppState {
            right_panel: PanelState {
                tabs: select_prev_element(state.right_panel.current_tab, state.right_panel.tabs),
                ..state.right_panel
            },
            ..state
        }
    } else {
        AppState { ..state }
    }
}

fn select_next_element<TFileSystem: Clone + Debug + Default + FileSystem>(
    current_tab: usize,
    items: Vec<TabState<TFileSystem>>,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for (idx, val) in items.iter().enumerate() {
        let filtered_items = val.filtered_items();
        if idx == current_tab && filtered_items.is_empty() == false {
            let next_tab = match val.tab_state.selected() {
                Some(current) => {
                    if current >= filtered_items.len() - 1 {
                        0
                    } else {
                        current + 1
                    }
                }
                None => 0,
            };
            let mut tab_state = val.tab_state.clone();
            tab_state.select(Some(next_tab));
            result.push(TabState {
                tab_state,
                selected: vec![filtered_items[next_tab].clone()],
                ..val.clone()
            })
        } else {
            result.push(val.clone());
        }
    }

    result
}

fn select_prev_element<TFileSystem: Clone + Debug + Default + FileSystem>(
    current_tab: usize,
    items: Vec<TabState<TFileSystem>>,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for (idx, val) in items.iter().enumerate() {
        let filtered_items = val.filtered_items();
        if idx == current_tab && filtered_items.is_empty() == false {
            let prev_tab = match val.tab_state.selected() {
                Some(current) => {
                    if current == 0 {
                        val.filtered_items().len() - 1
                    } else {
                        current - 1
                    }
                }
                None => 0,
            };
            let mut tab_state = val.tab_state.clone();
            tab_state.select(Some(prev_tab));
            result.push(TabState {
                tab_state,
                selected: vec![filtered_items[prev_tab].clone()],
                ..val.clone()
            });
        } else {
            result.push(val.clone());
        }
    }

    result
}

fn select_multiple_next_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    current_tab: usize,
    tabs: Vec<TabState<TFileSystem>>,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for (idx, val) in tabs.iter().enumerate() {
        let filtered_items = val.filtered_items();
        if idx == current_tab && filtered_items.is_empty() == false {
            let next_tab = match val.tab_state.selected() {
                Some(current) => {
                    if current >= filtered_items.len() - 1 {
                        0
                    } else {
                        current + 1
                    }
                }
                None => 0,
            };

            let mut tab_state = val.tab_state.clone();
            tab_state.select(Some(next_tab));

            let mut selected = val.selected.clone();
            selected.push(filtered_items[next_tab].clone());

            result.push(TabState {
                tab_state,
                selected,
                ..val.clone()
            });
        } else {
            result.push(val.clone());
        }
    }

    result
}

fn select_multiple_prev_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    current_tab: usize,
    tabs: Vec<TabState<TFileSystem>>,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for (idx, val) in tabs.iter().enumerate() {
        let filtered_items = val.filtered_items();
        if idx == current_tab && filtered_items.is_empty() == false {
            let prev_tab = match val.tab_state.selected() {
                Some(current) => {
                    if current == 0 {
                        val.filtered_items().len() - 1
                    } else {
                        current - 1
                    }
                }
                None => 0,
            };

            let mut tab_state = val.tab_state.clone();
            tab_state.select(Some(prev_tab));

            let mut selected = val.selected.clone();
            selected.push(filtered_items[prev_tab].clone());

            result.push(TabState {
                tab_state,
                selected,
                ..val.clone()
            });
        } else {
            result.push(val.clone());
        }
    }

    result
}

fn clear_selections_in_tab<TFileSystem: Clone + Debug + Default + FileSystem>(
    current_tab: usize,
    tabs: Vec<TabState<TFileSystem>>,
) -> Vec<TabState<TFileSystem>> {
    let mut result = Vec::<TabState<TFileSystem>>::new();
    for (idx, val) in tabs.iter().enumerate() {
        let filtered_items = val.filtered_items();
        if idx == current_tab && filtered_items.is_empty() == false {
            let mut tab_state = val.tab_state.clone();
            tab_state.select(None);
            result.push(TabState {
                tab_state,
                selected: Vec::new(),
                ..val.clone()
            });
        } else {
            result.push(val.clone());
        }
    }

    result
}
