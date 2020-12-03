use tui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders},
};

use crate::{
    app::{
        actions::FileManagerActions,
        state::{AppState, TabState},
    },
    core::{
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
    },
};

type TabComponentProps<'props> = TabState<'props>;

#[derive(Clone, Copy, Debug)]
pub struct TabComponentState {
    current_selected: usize,
    max_lenght: usize,
}

impl Default for TabComponentState {
    fn default() -> Self {
        TabComponentState::new(0, 0)
    }
}

impl TabComponentState {
    pub fn new(current_selected: usize, max_lenght: usize) -> Self {
        TabComponentState {
            current_selected,
            max_lenght,
        }
    }
}

pub struct TabComponent<'component> {
    base: ComponentBase<TabComponentProps<'component>, TabComponentState>,
}

impl<'component> TabComponent<'component> {
    pub fn new(
        props: Option<TabComponentProps<'component>>,
        state: Option<TabComponentState>,
    ) -> Self {
        TabComponent {
            base: ComponentBase::new(props, state),
        }
    }

    pub fn with_props(props: TabComponentProps<'component>) -> Self {
        TabComponent::new(Some(props), None)
    }

    pub fn empty() -> Self {
        TabComponent::new(None, None)
    }
}

impl<'component> Component<Event, AppState<'_>, FileManagerActions> for TabComponent<'component> {
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState, FileManagerActions>,
    ) -> bool {
        false
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>, area: Option<Rect>) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default())
            .border_type(tui::widgets::BorderType::Thick)
            .style(Style::default());
        frame.render_widget(block, area.unwrap())
    }
}
