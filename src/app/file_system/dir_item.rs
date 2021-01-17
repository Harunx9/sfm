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
}

impl DirItem {
    pub fn new(
        name: String,
        path: PathBuf,
        last_modification: DateTime<Local>,
        icon: String,
    ) -> Self {
        DirItem {
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
