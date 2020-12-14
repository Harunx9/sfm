use chrono::{DateTime, Local};

use super::config::Config;

pub mod directory;

#[derive(Clone, Debug)]
pub enum FileSystemItem {
    Directory(DirectoryItem),
    File(FileItem),
    Unknown,
}

impl FileSystemItem {
    pub fn get_name(&self) -> String {
        match self {
            FileSystemItem::Directory(dir) => dir.name.clone(),
            FileSystemItem::File(file) => file.name.clone(),
            FileSystemItem::Unknown => "".to_string(),
        }
    }
}

impl ToString for FileSystemItem {
    fn to_string(&self) -> String {
        match self {
            FileSystemItem::Directory(dir) => dir.to_string(),
            FileSystemItem::File(file) => file.to_string(),
            FileSystemItem::Unknown => "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DirectoryItem {
    is_visible: bool,
    name: String,
    path: String,
    last_modification: DateTime<Local>,
    icon: String,
}

impl DirectoryItem {
    pub fn new(
        name: String,
        path: String,
        is_visible: bool,
        last_modification: DateTime<Local>,
        icon: String,
    ) -> Self {
        DirectoryItem {
            is_visible,
            name,
            path,
            last_modification,
            icon,
        }
    }
}

impl ToString for DirectoryItem {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.name, self.last_modification.format()
    }
}

#[derive(Clone, Debug)]
pub struct FileItem {
    is_visible: bool,
    name: String,
    path: String,
    last_modification: DateTime<Local>,
}

impl FileItem {
    pub fn new(
        name: String,
        path: String,
        is_visible: bool,
        last_modification: DateTime<Local>,
    ) -> Self {
        FileItem {
            is_visible,
            name,
            path,
            last_modification,
        }
    }
}

impl ToString for FileItem {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.last_modification)
    }
}
