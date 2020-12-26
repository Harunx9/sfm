use chrono::{DateTime, Local};
use fs::Metadata;

use crate::app::config::icon_cfg::IconsConfig;

use super::{DirectoryItem, FileItem, FileSystemItem, SymlinkItem};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Clone, Debug)]
pub struct DirInfo {
    pub name: String,
    pub path: PathBuf,
}

impl DirInfo {
    pub fn new(path: &Path) -> Option<Self> {
        if let Ok(path_buffer) = fs::canonicalize(path) {
            let name = if let Some(file_name) = path_buffer.file_name() {
                file_name.to_str().unwrap_or("")
            } else {
                ""
            };
            let path = path_buffer.as_path().to_str().unwrap_or("");
            return Some(DirInfo {
                name: name.to_string(),
                path: PathBuf::from(path),
            });
        }
        None
    }
}

pub fn get_items_from_dir(dir: &Path, icons: &IconsConfig) -> Vec<FileSystemItem> {
    match fs::read_dir(dir) {
        Ok(mut iter) => {
            let mut result = Vec::new();
            while let Some(load_result) = iter.next() {
                if let Ok(dir_entry) = load_result {
                    result.push(map_dir_entry_to_file_system_item(dir_entry, icons));
                }
            }

            result.sort_by(|one, two| one.get_name().cmp(&two.get_name()));

            result
        }
        Err(_) => Vec::new(),
    }
}

fn map_dir_entry_to_file_system_item(dir_entry: DirEntry, icons: &IconsConfig) -> FileSystemItem {
    if let Ok(metadata) = dir_entry.metadata() {
        let (name, path, modified) = get_file_system_item_props(dir_entry, &metadata);
        let file_type = metadata.file_type();
        if file_type.is_file() {
            let file_extensions = name.split('.').last().unwrap_or("");
            return FileSystemItem::File(FileItem::new(
                name.to_string(),
                path,
                modified,
                icons.get_file_icon(file_extensions.to_string()),
            ));
        }

        if file_type.is_dir() {
            return FileSystemItem::Directory(DirectoryItem::new(
                name.to_string(),
                path,
                modified,
                icons.get_dir_icon(name),
            ));
        }

        if file_type.is_symlink() {
            let file_extensions = name.split('.').last().unwrap_or("");
            return FileSystemItem::Symlink(SymlinkItem::new(
                name.to_string(),
                path,
                modified,
                icons.get_file_icon(file_extensions.to_string()),
            ));
        }

        FileSystemItem::Unknown
    } else {
        FileSystemItem::Unknown
    }
}

fn get_file_system_item_props(
    dir_entry: DirEntry,
    metadata: &Metadata,
) -> (String, PathBuf, DateTime<Local>) {
    let modified: DateTime<Local> = if let Ok(last_modified) = metadata.modified() {
        last_modified.into()
    } else {
        SystemTime::now().into()
    };

    let entry_name = dir_entry.file_name();
    let name = if let Some(name) = entry_name.to_str() {
        name
    } else {
        ""
    };
    let path_buffer = dir_entry.path();

    (name.to_string(), path_buffer, modified)
}
