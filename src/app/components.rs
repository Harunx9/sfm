use crossterm::event::KeyCode;
use tui::backend::Backend;

use crate::core::{
    events::Event,
    store::Store,
    ui::{Component, ComponentBase},
};

use super::{
    actions::{AppActions, FrActions},
    state::{AppState, TabState},
};

pub struct RootComponent {}

impl RootComponent {
    pub fn new() -> Self {
        RootComponent {}
    }
}

impl Component<Event, (), AppState, FrActions> for RootComponent {
    fn handle_event(&mut self, event: Event, store: &mut Store<AppState, FrActions>) -> bool {
        if let Event::Keyboard(key_evt) = event {
            if let KeyCode::Char('q') = key_evt.code {
                store.dispatch(FrActions::App(AppActions::Exit));
                return true;
            }
        }

        false
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>) {}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TabProps {
    current_selected: u32,
    max_lenght: u32,
}

pub struct TabComponent {
    base: ComponentBase<TabProps, TabState>,
}

impl Component<Event, TabState, AppState, FrActions> for TabComponent {
    fn handle_event(&mut self, event: Event, store: &mut Store<AppState, FrActions>) -> bool {
        false
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>) {}
}
