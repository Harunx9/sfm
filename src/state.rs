pub struct State {
    left_tab: Tab,
    right_tab: Tab,
    pub app_exit: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            left_tab: Tab::default(),
            right_tab: Tab::default(),
            app_exit: false,
        }
    }
}

pub struct Tab {
    title: String,
    items: Vec<FileSystemItem>,
}

impl Default for Tab {
    fn default() -> Self {
        Tab {
            title: String::new(),
            items: Vec::new(),
        }
    }
}

pub enum FileSystemItem {
    Directory(DirectoryItem),
    File(FileItem),
}

pub struct DirectoryItem {
    is_visible: bool,
    name: String,
    path: String,
    icon: char,
}

pub struct FileItem {
    is_visible: bool,
    name: String,
    path: String,
    icon: char,
}

pub trait Command<TArgs>
where
    TArgs: Sized,
{
    fn execute_on(file_system_item: &FileSystemItem, args: TArgs) {}
}
