use tui::{layout::Rect, text::Spans};

pub mod color_scheme;
pub mod config;
pub mod events;
pub mod key_binding;
pub mod store;
pub mod ui;

pub trait ToSpans {
    fn to_spans(&self, area: Rect, show_icons: bool) -> Spans;
}
