use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    app::{
        actions::{AppAction, FileManagerActions},
        state::AppState,
    },
    core::{events::Event, store::Store, ui::component::Component},
};

use super::tab::TabComponent;

pub struct RootComponent {
    left_tab: TabComponent,
    right_tab: TabComponent,
}

impl RootComponent {
    pub fn new() -> Self {
        RootComponent {
            left_tab: TabComponent::empty(),
            right_tab: TabComponent::empty(),
        }
    }
}

impl Component<Event, AppState, FileManagerActions> for RootComponent {
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState, FileManagerActions>,
    ) -> bool {
        let mut result = false;
        if let Event::Keyboard(key_evt) = event {
            if let KeyCode::Char('q') = key_evt.code {
                store.dispatch(FileManagerActions::App(AppAction::Exit));
                result = true
            }
        }

        result = self.left_tab.handle_event(event, store);
        if result == true {
            return result;
        }
        result = self.right_tab.handle_event(event, store);

        result
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>, area: Option<Rect>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());
        self.left_tab.render(frame, Some(layout[0]));
        self.right_tab.render(frame, Some(layout[1]));
    }
}
