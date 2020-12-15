use crate::app::{actions::DirectoryAction, state::AppState};

pub fn dir_reducer(state: AppState, dir_action: DirectoryAction) -> AppState {
    match dir_action {
        DirectoryAction::Delete { path } => state,
        DirectoryAction::Rename { from, to } => state,
        DirectoryAction::Move { from, to } => state,
        DirectoryAction::Open { path } => state,
        DirectoryAction::Navigate { to } => state,
    }
}
