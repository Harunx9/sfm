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

use super::panel::PanelComponent;

pub struct RootComponent {
    left_panel: PanelComponent,
    right_panel: PanelComponent,
}

impl RootComponent {
    pub fn new() -> Self {
        RootComponent {
            left_panel: PanelComponent::empty(),
            right_panel: PanelComponent::empty(),
        }
    }
}

impl Component<Event, AppState, FileManagerActions> for RootComponent {
    fn on_init(&mut self, store: &Store<AppState, FileManagerActions>) {
        let state = store.get_state();
        self.left_panel = PanelComponent::from(state.left_panel);
        self.right_panel = PanelComponent::from(state.right_panel);
    }

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

        result = self.left_panel.handle_event(event, store);
        if result == true {
            return result;
        }
        result = self.right_panel.handle_event(event, store);

        result
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>, _area: Option<Rect>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());
        self.left_panel.render(frame, Some(layout[0]));
        self.right_panel.render(frame, Some(layout[1]));
    }
}
