use tui::{backend::Backend, Frame};

use super::store::Store;

pub struct ComponentBase<TProps, TState>
where
    TProps: Default + Clone,
    TState: Default + Clone,
{
    props: Option<TProps>,
    state: Option<TState>,
}

impl<TProps, TState> Default for ComponentBase<TProps, TState>
where
    TProps: Default + Clone,
    TState: Default + Clone,
{
    fn default() -> Self {
        ComponentBase {
            props: None,
            state: None,
        }
    }
}

impl<TProps, TState> ComponentBase<TProps, TState>
where
    TProps: Default + Clone,
    TState: Default + Clone,
{
    pub fn get_props(&self) -> Option<TProps> {
        self.props.clone()
    }

    pub fn get_state(&self) -> Option<TState> {
        self.state.clone()
    }

    pub fn set_state(&mut self, callback: StateSetter<TState>) {
        self.state = callback(self.state.clone());
    }
}

pub trait Component<TEvent, TProps, TGlobalState, TAction>
where
    TEvent: Clone,
    TProps: Clone,
    TGlobalState: Default + Clone,
    TAction: Clone,
{
    fn get_props(&self) -> Option<TProps> {
        None
    }
    fn set_props(&mut self, _props: TProps) {}
    fn handle_event(&mut self, event: TEvent, store: &mut Store<TGlobalState, TAction>) -> bool;
    fn render<TBackend: Backend>(&self, frame: &mut Frame<TBackend>);
}

type StateSetter<TState> = fn(Option<TState>) -> Option<TState>;
