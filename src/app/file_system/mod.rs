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

impl ToSpans for FileSystemItem {
    fn to_spans(&self, area: Rect) -> Spans {
        match self {
            FileSystemItem::Directory(dir) => dir.to_spans(area),
            FileSystemItem::File(file) => file.to_spans(area),
            FileSystemItem::Unknown => Spans::default(),
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

impl ToSpans for DirectoryItem {
    fn to_spans(&self, area: Rect) -> Spans {
        let width = area.width;

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
    is_visible: bool,
    name: String,
    path: String,
    last_modification: DateTime<Local>,
    icon: String,
}

impl FileItem {
    pub fn new(
        name: String,
        path: String,
        is_visible: bool,
        last_modification: DateTime<Local>,
        icon: String,
    ) -> Self {
        FileItem {
            is_visible,
            name,
            path,
            last_modification,
            icon,
        }
    }
}

impl ToSpans for FileItem {
    fn to_spans(&self, area: Rect) -> Spans {
        let width = area.width;

        Spans::from(vec![
            Span::from("  "),
            Span::from(self.icon.clone()),
            Span::from("  "),
            Span::from(self.name.clone()),
        ])
    }
}
