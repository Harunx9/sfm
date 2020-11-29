use tui::{backend::Backend, style::Style, widgets::Widget, Frame};

pub struct ComponentBase<TProps, TState>
where
    TProps: Clone + Copy,
    TState: Clone + Copy,
{
    props: Option<TProps>,
    state: Option<TState>,
}

impl<TProps, TState> ComponentBase<TProps, TState>
where
    TProps: Clone + Copy,
    TState: Clone + Copy,
{
    pub fn get_props(&self) -> Option<TProps> {
        self.props
    }

    pub fn get_state(&self) -> Option<TState> {
        self.state
    }

    pub fn set_state(&mut self, callback: StateSetter<TState>) {
        self.state = callback(self.state);
    }
}

pub trait Component<TBackend, TEvent>
where
    TBackend: Backend,
    TEvent: Clone + Copy,
{
    fn handle_event(event: TEvent) -> bool;
    fn render(frame: Frame<TBackend>);
}

type StateSetter<TState> = fn(Option<TState>) -> Option<TState>;
