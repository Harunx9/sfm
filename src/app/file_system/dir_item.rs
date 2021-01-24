use std::path::PathBuf;

use chrono::{DateTime, Local};
use tui::{
    layout::Rect,
    text::{Span, Spans},
};

use crate::core::ToSpans;

#[derive(Clone, Debug)]
pub struct DirItem {
    name: String,
    path: PathBuf,
    last_modification: DateTime<Local>,
    icon: String,
    is_empty: bool,
}

impl DirItem {
    pub fn new(
        name: String,
        path: PathBuf,
        last_modification: DateTime<Local>,
        icon: String,
        is_empty: bool,
    ) -> Self {
        DirItem {
            name,
            path,
            last_modification,
            icon,
            is_empty,
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

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}

impl ToSpans for DirItem {
    fn to_spans(&self, _area: Rect, show_icons: bool) -> Spans {
        if show_icons {
            Spans::from(vec![
                Span::from("  "),
                Span::from(self.icon.clone()),
                Span::from("  "),
                Span::from(self.name.clone()),
            ])
        } else {
            Spans::from(vec![Span::from("  "), Span::from(self.name.clone())])
        }
    }
}
