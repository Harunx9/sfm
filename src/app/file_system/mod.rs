use std::path::PathBuf;

use chrono::{DateTime, Local};
use tui::{
    layout::Rect,
    text::{Span, Spans},
};

use crate::core::ToSpans;

pub mod directory;

#[derive(Clone, Debug)]
pub enum FileSystemItem {
    Directory(DirectoryItem),
    File(FileItem),
    Symlink(SymlinkItem),
    Unknown,
}

impl FileSystemItem {
    pub fn get_path(&self) -> PathBuf {
        match self {
            FileSystemItem::Directory(dir) => dir.path.clone(),
            FileSystemItem::File(file) => file.path.clone(),
            FileSystemItem::Symlink(symlink) => symlink.file_path.clone(),
            FileSystemItem::Unknown => PathBuf::new(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            FileSystemItem::Directory(dir) => dir.name.clone(),
            FileSystemItem::File(file) => file.name.clone(),
            FileSystemItem::Symlink(symlink) => symlink.name.clone(),
            FileSystemItem::Unknown => "".to_string(),
        }
    }

    pub fn is_symlink(&self) -> bool {
        match self {
            FileSystemItem::Directory(_) => false,
            FileSystemItem::File(_) => false,
            FileSystemItem::Symlink(_) => true,
            FileSystemItem::Unknown => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileSystemItem::Directory(_) => false,
            FileSystemItem::File(_) => true,
            FileSystemItem::Symlink(_) => false,
            FileSystemItem::Unknown => false,
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            FileSystemItem::Directory(_) => true,
            FileSystemItem::File(_) => false,
            FileSystemItem::Symlink(_) => false,
            FileSystemItem::Unknown => false,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            FileSystemItem::Directory(dir) => dir.is_visible(),
            FileSystemItem::File(file) => file.is_visible(),
            FileSystemItem::Symlink(_) => true,
            FileSystemItem::Unknown => false,
        }
    }
}

impl ToSpans for FileSystemItem {
    fn to_spans(&self, area: Rect) -> Spans {
        match self {
            FileSystemItem::Directory(dir) => dir.to_spans(area),
            FileSystemItem::File(file) => file.to_spans(area),
            FileSystemItem::Symlink(symlink) => symlink.to_spans(area),
            FileSystemItem::Unknown => Spans::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SymlinkItem {
    name: String,
    file_path: PathBuf,
    last_modification: DateTime<Local>,
    icon: String,
}

impl SymlinkItem {
    pub fn new(
        name: String,
        file_path: PathBuf,
        last_modification: DateTime<Local>,
        icon: String,
    ) -> Self {
        Self {
            name,
            file_path,
            last_modification,
            icon,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_path(&self) -> PathBuf {
        self.file_path.clone()
    }

    pub fn is_visible(&self) -> bool {
        self.name.starts_with('.')
    }
}

impl ToSpans for SymlinkItem {
    fn to_spans(&self, _area: Rect) -> Spans {
        Spans::from(vec![
            Span::from("  "),
            Span::from(self.icon.clone()),
            Span::from("  "),
            Span::from(self.name.clone()),
            Span::from(" -> "),
            Span::from(self.file_path.to_str().unwrap_or("")),
        ])
    }
}

#[derive(Clone, Debug)]
pub struct DirectoryItem {
    name: String,
    path: PathBuf,
    last_modification: DateTime<Local>,
    icon: String,
}

impl DirectoryItem {
    pub fn new(
        name: String,
        path: PathBuf,
        last_modification: DateTime<Local>,
        icon: String,
    ) -> Self {
        DirectoryItem {
            name,
            path,
            last_modification,
            icon,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn is_visible(&self) -> bool {
        self.name.starts_with('.')
    }
}

impl ToSpans for DirectoryItem {
    fn to_spans(&self, _area: Rect) -> Spans {
        Spans::from(vec![
            Span::from("  "),
            Span::from(self.icon.clone()),
            Span::from("  "),
            Span::from(self.name.clone()),
        ])
    }
}

#[derive(Clone, Debug)]
pub struct FileItem {
    name: String,
    path: PathBuf,
    last_modification: DateTime<Local>,
    icon: String,
}

impl FileItem {
    pub fn new(
        name: String,
        path: PathBuf,
        last_modification: DateTime<Local>,
        icon: String,
    ) -> Self {
        FileItem {
            name,
            path,
            last_modification,
            icon,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn is_visible(&self) -> bool {
        self.name.starts_with('.')
    }
}

impl ToSpans for FileItem {
    fn to_spans(&self, _area: Rect) -> Spans {
        Spans::from(vec![
            Span::from("  "),
            Span::from(self.icon.clone()),
            Span::from("  "),
            Span::from(self.name.clone()),
        ])
    }
}
