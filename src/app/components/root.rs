use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    app::{
        actions::{AppAction, FileManagerActions, PanelSide},
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

    fn map_state(&mut self, store: &Store<AppState, FileManagerActions>) {
        let state = store.get_state();
        self.left_panel = PanelComponent::with_panel_state(state.left_panel, PanelSide::Left);
        self.right_panel = PanelComponent::with_panel_state(state.right_panel, PanelSide::Right);
    }
}

impl Component<Event, AppState, FileManagerActions> for RootComponent {
    fn on_init(&mut self, store: &Store<AppState, FileManagerActions>) {
        self.map_state(store);
    }

    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        if let Event::Keyboard(key_evt) = event {
            if state.config.keyboard_cfg.quit.is_pressed(key_evt) {
                store.dispatch(FileManagerActions::App(AppAction::Exit));
                return true;
            }

            if state
                .config
                .keyboard_cfg
                .focus_left_panel
                .is_pressed(key_evt)
            {
                store.dispatch(FileManagerActions::App(AppAction::FocusLeft));
                self.map_state(store);
                return true;
            }

            if state
                .config
                .keyboard_cfg
                .focus_right_panel
                .is_pressed(key_evt)
            {
                store.dispatch(FileManagerActions::App(AppAction::FocusRight));
                self.map_state(store);
                return true;
            }
        }

        let mut result = self.left_panel.handle_event(event, store);
        if result == true {
            self.map_state(store);
            return result;
        }
        result = self.right_panel.handle_event(event, store);
        self.map_state(store);

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
