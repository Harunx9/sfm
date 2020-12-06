use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::{
    app::{
        actions::FileManagerActions,
        state::{AppState, PanelState, TabState},
    },
    core::{
        events::Event,
        ui::{component::Component, component_base::ComponentBase},
    },
};

use super::tab::{TabComponent, TabComponentProps};

#[derive(Clone, Default, Debug)]
pub struct PanelComponentProps {
    tabs: Vec<String>,
    current_tab: usize,
}

pub struct PanelComponent {
    base: ComponentBase<PanelComponentProps, ()>,
    tab: TabComponent,
}

impl PanelComponent {
    pub fn new(props: PanelComponentProps, tab_props: TabComponentProps) -> Self {
        PanelComponent {
            base: ComponentBase::new(Some(props), None),
            tab: TabComponent::with_props(tab_props),
        }
    }
}

impl Component<Event, AppState, FileManagerActions> for PanelComponent {
    fn render<TBackend: Backend>(&self, frame: &mut Frame<TBackend>, area: Option<Rect>) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area.unwrap());

        let tabs = u
    }
}

pub fn map_global_state_to_panel_props(gloal_state: PanelState) -> PanelComponentProps {
    let tabs = gloal_state.tabs.iter().map(|tab| tab.name).collect();
    PanelComponentProps {
        tabs,
        current_tab: gloal_state.current_tab,
    }
}
