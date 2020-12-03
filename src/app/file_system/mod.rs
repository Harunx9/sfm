use chrono::{DateTime, Local};

pub mod directory;

#[derive(Clone, Debug)]
pub enum FileSystemItem<'item> {
    Directory(DirectoryItem<'item>),
    File(FileItem<'item>),
    Unknown,
}

impl<'item> ToString for FileSystemItem<'item> {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct DirectoryItem<'item> {
    is_visible: bool,
    name: &'item str,
    path: &'item str,
    last_modification: DateTime<Local>,
}

impl<'item> ToString for DirectoryItem<'item> {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.last_modification)
    }
}

#[derive(Clone, Debug)]
pub struct FileItem<'item> {
    is_visible: bool,
    name: &'item str,
    path: &'item str,
    last_modification: DateTime<Local>,
}

impl<'item> ToString for FileItem<'item> {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.last_modification)
    }
}
