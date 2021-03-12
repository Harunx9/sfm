use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

pub mod create_modal;
pub mod error_modal;
pub mod not_empty_dir_delete_modal;
pub mod panel;
pub mod rename_modal;
pub mod root;
pub mod tab;

fn create_modal_layout(x_percent: u16, y_percent: u16, rect: Rect) -> Rect {
    let vertical_slice = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - y_percent) / 2),
                Constraint::Percentage(y_percent),
                Constraint::Percentage((100 - y_percent) / 2),
            ]
            .as_ref(),
        )
        .split(rect);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - x_percent) / 2),
                Constraint::Percentage(x_percent),
                Constraint::Percentage((100 - x_percent) / 2),
            ]
            .as_ref(),
        )
        .split(vertical_slice[1])[1]
}

#[derive(Clone)]
pub struct ModalStyle {
    pub border_color: Color,
    pub selected_element_background: Color,
    pub selected_element_foreground: Color,
}

impl ModalStyle {
    pub fn new(
        border_color: Color,
        selected_element_background: Color,
        selected_element_foreground: Color,
    ) -> Self {
        Self {
            border_color,
            selected_element_background,
            selected_element_foreground,
        }
    }
}

impl Default for ModalStyle {
    fn default() -> Self {
        ModalStyle {
            border_color: Color::Red,
            selected_element_background: Color::Yellow,
            selected_element_foreground: Color::Black,
        }
    }
}
