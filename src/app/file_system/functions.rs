#[cfg(unix)]
use std::os::unix::fs;
#[cfg(windows)]
use std::os::windows::fs;

use std::{
    fs::{read_link, DirEntry, Metadata},
    io,
    path::{Path, PathBuf},
    time::SystemTime,
};

use chrono::{DateTime, Local};

use crate::app::config::icon_cfg::IconsConfig;

use super::{
    dir_item::DirItem, file_item::FileItem, file_system_item::FileSystemItem,
    symlink_item::SymlinkItem,
};

#[cfg(unix)]
pub fn create_link<TPath: AsRef<Path>>(symlink_path: TPath, item_path: TPath) -> io::Result<()> {
    let symlink_path = expand_if_contains_tilde(symlink_path).unwrap();
    fs::symlink(item_path, symlink_path)
}

#[cfg(windows)]
pub fn create_link<TPath: AsRef<Path>>(symlink_path: TPath, item_path: TPath) -> io::Result<()> {
    let symlink_path = expand_if_contains_tilde(symlink_path).unwrap();
    if item_path.is_dir() {
        fs::symlink_dir(item_path, symlink_path)
    } else {
        fs::symlink_file(item_path, symlink_path)
    }
}

//From: https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
pub fn expand_if_contains_tilde<TPath: AsRef<Path>>(input: TPath) -> Option<PathBuf> {
    let path = input.as_ref();
    if path.starts_with("~") == false {
        return Some(path.to_path_buf());
    }
    if path == Path::new("~") {
        return dirs::home_dir();
    }

    dirs::home_dir().map(|mut home_path| {
        if home_path == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            path.strip_prefix("~").unwrap().to_path_buf()
        } else {
            home_path.push(path.strip_prefix("~/").unwrap());
            home_path
        }
    })
}

pub fn map_dir_entry_to_file_system_item(
    dir_entry: DirEntry,
    icons: &IconsConfig,
) -> FileSystemItem {
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
            return FileSystemItem::Directory(DirItem::new(
                name.to_string(),
                path.clone(),
                modified,
                icons.get_dir_icon(name),
                path.read_dir()
                    .map(|mut i| i.next().is_none())
                    .unwrap_or(false),
            ));
        }

        if file_type.is_symlink() {
            let file_extensions = name.split('.').last().unwrap_or("");
            match read_link(path.clone()) {
                Ok(target) => {
                    return FileSystemItem::Symlink(SymlinkItem::new(
                        name.to_string(),
                        path,
                        target.clone(),
                        modified,
                        if target.is_file() {
                            icons.get_file_icon(file_extensions.to_string())
                        } else {
                            icons.get_dir_icon(name)
                        },
                    ))
                }
                Err(_) => {
                    return FileSystemItem::Symlink(SymlinkItem::new(
                        name.to_string(),
                        path.clone(),
                        path,
                        modified,
                        icons.get_file_icon(file_extensions.to_string()),
                    ))
                }
            }
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
