use std::path::PathBuf;

use tui::{layout::Rect, text::Spans};

use crate::core::ToSpans;

use super::{dir_item::DirItem, file_item::FileItem, symlink_item::SymlinkItem};

#[derive(Clone, Debug)]
pub enum FileSystemItem {
    Directory(DirItem),
    File(FileItem),
    Symlink(SymlinkItem),
    Unknown,
}

impl FileSystemItem {
    pub fn get_path(&self) -> PathBuf {
        match self {
            FileSystemItem::Directory(dir) => dir.get_path(),
            FileSystemItem::File(file) => file.get_path(),
            FileSystemItem::Symlink(symlink) => symlink.get_path(),
            FileSystemItem::Unknown => PathBuf::new(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            FileSystemItem::Directory(dir) => dir.get_name(),
            FileSystemItem::File(file) => file.get_name(),
            FileSystemItem::Symlink(symlink) => symlink.get_name(),
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
    fn to_spans(&self, area: Rect, show_icons: bool) -> Spans {
        match self {
            FileSystemItem::Directory(dir) => dir.to_spans(area, show_icons),
            FileSystemItem::File(file) => file.to_spans(area, show_icons),
            FileSystemItem::Symlink(symlink) => symlink.to_spans(area, show_icons),
            FileSystemItem::Unknown => Spans::default(),
        }
    }
}
