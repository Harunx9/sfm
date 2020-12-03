use super::file_system::FileSystemItem;

#[derive(Clone, Debug)]
pub struct AppState<'state> {
    pub left_tab: TabState<'state>,
    pub right_tab: TabState<'state>,
    pub app_exit: bool,
}

impl<'state> Default for AppState<'state> {
    fn default() -> Self {
        AppState {
            left_tab: TabState::default(),
            right_tab: TabState::default(),
            app_exit: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TabState<'tab> {
    pub title: String,
    pub items: Vec<FileSystemItem<'tab>>,
}

impl<'tab> Default for TabState<'tab> {
    fn default() -> Self {
        TabState {
            title: String::new(),
            items: Vec::new(),
        }
    }
}
