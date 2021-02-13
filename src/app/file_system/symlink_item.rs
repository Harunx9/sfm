use std::path::PathBuf;

use chrono::{DateTime, Local};
use tui::{
    layout::Rect,
    text::{Span, Spans},
};

use crate::core::ToSpans;

#[derive(Clone, Debug)]
pub struct SymlinkItem {
    name: String,
    path: PathBuf,
    target: PathBuf,
    last_modification: DateTime<Local>,
    icon: String,
}

impl SymlinkItem {
    pub fn new(
        name: String,
        path: PathBuf,
        target: PathBuf,
        last_modification: DateTime<Local>,
        icon: String,
    ) -> Self {
        Self {
            name,
            path,
            target,
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

impl ToSpans for SymlinkItem {
    fn to_spans(&self, _area: Rect, show_icons: bool) -> Spans {
        if show_icons {
            Spans::from(vec![
                Span::from("  "),
                Span::from(self.icon.clone()),
                Span::from("  "),
                Span::from(
                    self.last_modification
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(),
                ),
                Span::from("  "),
                Span::from(self.name.clone()),
                Span::from(" -> "),
                Span::from(self.target.to_str().unwrap_or("")),
            ])
        } else {
            Spans::from(vec![
                Span::from("  "),
                Span::from(
                    self.last_modification
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(),
                ),
                Span::from("  "),
                Span::from(self.name.clone()),
                Span::from(" -> "),
                Span::from(self.target.to_str().unwrap_or("")),
            ])
        }
    }
}
