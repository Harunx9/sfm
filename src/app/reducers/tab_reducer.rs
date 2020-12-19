use crate::app::{
    actions::TabAction,
    state::{AppState, PanelState, TabState},
};

pub fn tab_reducer(state: AppState, tab_action: TabAction) -> AppState {
    match tab_action {
        TabAction::Next => select_next(state),
        TabAction::Previous => select_previous(state),
    }
}

fn select_next(state: AppState) -> AppState {
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

fn select_previous(state: AppState) -> AppState {
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

fn select_next_element(current_tab: usize, mut items: Vec<TabState>) -> Vec<TabState> {
    for (idx, val) in items.iter_mut().enumerate() {
        if idx == current_tab {
            let next_tab = match val.tab_state.selected() {
                Some(current) => {
                    if current >= val.items.len() - 1 {
                        0
                    } else {
                        current + 1
                    }
                }
                None => 0,
            };

            val.tab_state.select(Some(next_tab));
        }
    }

    items.clone()
}

fn select_prev_element(current_tab: usize, mut items: Vec<TabState>) -> Vec<TabState> {
    for (idx, val) in items.iter_mut().enumerate() {
        if idx == current_tab {
            let prev_tab = match val.tab_state.selected() {
                Some(current) => {
                    if current == 0 {
                        val.items.len() - 1
                    } else {
                        current - 1
                    }
                }
                None => 0,
            };

            val.tab_state.select(Some(prev_tab));
        }
    }

    items.clone()
}
