use tui::layout::{Constraint, Direction, Layout, Rect};

pub mod create_modal;
pub mod panel;
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
