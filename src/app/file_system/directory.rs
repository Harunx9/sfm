use chrono::{DateTime, Local};
use fs::Metadata;

use super::{DirectoryItem, FileItem, FileSystemItem};
use std::{
    fs::{self, DirEntry},
    time::SystemTime,
};

#[derive(Clone, Debug)]
pub struct DirInfo {
    pub name: String,
    pub path: String,
}

impl DirInfo {
    pub fn new(path: &str) -> Option<Self> {
        if let Ok(path_buffer) = fs::canonicalize(path) {
            let name = if let Some(file_name) = path_buffer.file_name() {
                file_name.to_str().unwrap_or("")
            } else {
                ""
            };
            let path = path_buffer.as_path().to_str().unwrap_or("");
            return Some(DirInfo {
                name: name.to_string(),
                path: path.to_string(),
            });
        }
        None
    }
}

pub fn get_items_from_dir(dir: &str) -> Vec<FileSystemItem> {
    match fs::read_dir(dir) {
        Ok(mut iter) => {
            let mut result = Vec::new();
            while let Some(load_result) = iter.next() {
                if let Ok(dir_entry) = load_result {
                    result.push(map_dir_entry_to_file_system_item(dir_entry));
                }
            }

            result
        }
        Err(_) => Vec::new(),
    }
}

fn map_dir_entry_to_file_system_item(dir_entry: DirEntry) -> FileSystemItem {
    if let Ok(metadata) = dir_entry.metadata() {
        let (name, path, modified) = get_file_system_item_props(dir_entry, &metadata);

        if metadata.is_file() {
            return FileSystemItem::File(FileItem::new(
                name.to_string(),
                path.to_string(),
                true,
                modified,
            ));
        }

        if metadata.is_dir() {
            return FileSystemItem::Directory(DirectoryItem::new(
                name.to_string(),
                path.to_string(),
                true,
                modified,
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
) -> (String, String, DateTime<Local>) {
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
    let path = if let Some(path) = path_buffer.to_str() {
        path
    } else {
        ""
    };

    (name.to_string(), path.to_string(), modified)
}
