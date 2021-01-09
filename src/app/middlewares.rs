use super::{
    actions::{
        AppAction, DirectoryAction, FileAction, FileManagerActions, PanelInfo, SymlinkAction,
    },
    state::{AppState, ModalType},
};
use crate::core::store::Store;
use std::fs;

pub fn symlink_middleware(
    store: &mut Store<AppState, FileManagerActions>,
    action: FileManagerActions,
) -> Option<FileManagerActions> {
    match action {
        FileManagerActions::Symlink(symlink_action) => symlink_resolver(store, symlink_action),
        _ => Some(action),
    }
}

fn symlink_resolver(
    _: &mut Store<AppState, FileManagerActions>,
    symlink_action: SymlinkAction,
) -> Option<FileManagerActions> {
    match symlink_action {
        SymlinkAction::Open { panel, in_new_tab } => match fs::read_link(panel.path.as_path()) {
            Ok(link_path) => {
                if link_path.is_dir() {
                    Some(FileManagerActions::Directory(DirectoryAction::Open {
                        panel: PanelInfo {
                            path: link_path,
                            tab: panel.tab,
                            side: panel.side,
                        },
                        in_new_tab,
                    }))
                } else {
                    Some(FileManagerActions::File(FileAction::Open {
                        panel: PanelInfo {
                            path: link_path,
                            tab: panel.tab,
                            side: panel.side,
                        },
                    }))
                }
            }
            Err(err) => Some(FileManagerActions::App(AppAction::ShowModal(
                ModalType::ErrorModal(format!("{}", err)),
            ))),
        },
        SymlinkAction::Delete { panel } => {
            if panel.path.is_dir() {
                Some(FileManagerActions::Directory(DirectoryAction::Delete {
                    panel,
                }))
            } else {
                Some(FileManagerActions::File(FileAction::Delete { panel }))
            }
        }
        _ => Some(FileManagerActions::Symlink(symlink_action)),
    }
}
