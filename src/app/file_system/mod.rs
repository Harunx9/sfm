use chrono::{DateTime, Local};

pub mod directory;

#[derive(Clone, Debug)]
pub enum FileSystemItem {
    Directory(DirectoryItem),
    File(FileItem),
    Unknown,
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
}

impl DirectoryItem {
    pub fn new(
        name: String,
        path: String,
        is_visible: bool,
        last_modification: DateTime<Local>,
    ) -> Self {
        DirectoryItem {
            is_visible,
            name,
            path,
            last_modification,
        }
    }
}

impl ToString for DirectoryItem {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.last_modification)
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
