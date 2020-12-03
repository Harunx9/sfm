pub mod directory;

#[derive(Clone, Debug)]
pub enum FileSystemItem<'item> {
    Directory(DirectoryItem<'item>),
    File(FileItem<'item>),
}

#[derive(Clone, Debug)]
pub struct DirectoryItem<'item> {
    is_visible: bool,
    name: &'item str,
    path: &'item str,
    icon: char,
}

#[derive(Clone, Debug)]
pub struct FileItem<'item> {
    is_visible: bool,
    name: &'item str,
    path: &'item str,
    icon: char,
}
