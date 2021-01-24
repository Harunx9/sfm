use super::{
    actions::{
        AppAction, DirectoryAction, FileAction, FileManagerActions, PanelInfo, SymlinkAction,
    },
    file_system::FileSystem,
    state::{AppState, ModalType},
};
use crate::core::store::Store;
use std::{fmt::Debug, fs};

pub fn symlink_middleware<TFileSystem: Clone + Debug + Default + FileSystem>(
    store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    action: FileManagerActions,
) -> Option<FileManagerActions> {
    match action {
        FileManagerActions::Symlink(symlink_action) => symlink_resolver(store, symlink_action),
        _ => Some(action),
    }
}

fn symlink_resolver<TFileSystem: Clone + Debug + Default + FileSystem>(
    _: &mut Store<AppState<TFileSystem>, FileManagerActions>,
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
        _ => Some(FileManagerActions::Symlink(symlink_action)),
    }
}

pub fn dir_middleware<TFileSystem: Clone + Debug + Default + FileSystem>(
    store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    action: FileManagerActions,
) -> Option<FileManagerActions> {
    match action {
        FileManagerActions::Directory(dir_action) => directory_resolver(store, dir_action),
        _ => Some(action),
    }
}

fn directory_resolver<TFileSystem: Clone + Debug + Default + FileSystem>(
    _: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    dir_action: DirectoryAction,
) -> Option<FileManagerActions> {
    match dir_action {
        DirectoryAction::Delete { panel, is_empty } => {
            if is_empty {
                Some(FileManagerActions::Directory(DirectoryAction::Delete {
                    panel,
                    is_empty,
                }))
            } else {
                Some(FileManagerActions::App(AppAction::ShowModal(
                    ModalType::DeleteDirWithContent {
                        panel_side: panel.side,
                        panel_tab: panel.tab,
                        path: panel.path,
                    },
                )))
            }
        }
        _ => Some(FileManagerActions::Directory(dir_action)),
    }
}
