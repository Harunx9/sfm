use crossterm::event::KeyCode;
use tui::backend::Backend;

use crate::core::{
    events::Event,
    store::Store,
    ui::{Component, ComponentBase},
};

use super::{
    actions::{AppActions, FrActions},
    state::State,
};

pub struct Root;

impl Root {
    pub fn new() -> Self {
        Root {}
    }
}

impl Component<Event, (), State, FrActions> for Root {
    fn handle_event(&mut self, event: Event, store: &mut Store<State, FrActions>) -> bool {
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
