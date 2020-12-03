use super::FileSystemItem;
use std::fs::{self, DirEntry};

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

fn map_dir_entry_to_file_system_item<'item>(dir_entry: DirEntry) -> FileSystemItem<'item> {
    if let Ok(metadata) = dir_entry.metadata() {
        if metadata.is_file() {
            return FileSystemItem::Unknown;
        }

        FileSystemItem::Unknown
    } else {
        FileSystemItem::Unknown
    }
}
