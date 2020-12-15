use crate::app::{actions::FileAction, state::AppState};

pub fn file_reducer(state: AppState, file_action: FileAction) -> AppState {
    match file_action {
        FileAction::Delete { path } => state,
        FileAction::Rename { from, to } => state,
        FileAction::Move { from, to } => state,
        FileAction::Open { path } => state,
    }
}
