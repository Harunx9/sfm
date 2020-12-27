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
    pub fn new(props: Option<TProps>, state: Option<TState>) -> Self {
        ComponentBase { props, state }
    }

    pub fn get_props(&self) -> Option<TProps> {
        self.props.clone()
    }

    pub fn get_state(&self) -> Option<TState> {
        self.state.clone()
    }

    pub fn set_state<StateSetter>(&mut self, callback: StateSetter)
    where
        StateSetter: Fn(TState) -> TState,
    {
        if let Some(state) = self.state.clone() {
            self.state = Some(callback(state.clone()))
        }
    }
}
