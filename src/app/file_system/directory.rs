use chrono::{DateTime, Local};
use fs::Metadata;

use super::{FileItem, FileSystemItem};
use std::{
    fs::{self, DirEntry},
    time::SystemTime,
};

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

        if metadata.is_file() {
            return FileSystemItem::File(FileItem::new(
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
