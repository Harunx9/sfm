use tui::backend::Backend;

pub struct WidgetBase<TProps, TState>
where
    TProps: Clone + Copy,
    TState: Clone + Copy,
{
    props: TProps,
    state: TState,
}

impl<TProps: Clone + Copy, TState: Clone + Copy> WidgetBase<TProps, TState> {
    pub fn get_props(&self) -> TProps {
        self.props
    }

    pub fn get_state(&self) -> TState {
        self.state
    }
}

pub struct Root<TState: Clone + Copy, TProps: Clone + Copy> {
    base: WidgetBase<TState, TProps>,
}

pub trait Component<TBackend, TEvent>
where
    TBackend: Backend,
{
    fn handle_event(event: TEvent) -> bool;
    fn render(backend: TBackend);
}
