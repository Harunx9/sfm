use crate::app::{actions::PanelAction, state::AppState};

pub fn panel_reducer(state: AppState, panel_action: PanelAction) -> AppState {
    match panel_action {
        PanelAction::Next => state,
        PanelAction::Previous => state,
    }
}
