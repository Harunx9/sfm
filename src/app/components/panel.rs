use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::{
    app::{
        actions::{FileManagerActions, PanelSide},
        state::{AppState, PanelState},
    },
    core::{
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
    },
};

use super::tab::{TabComponent, TabComponentProps};

#[derive(Clone, Default, Debug)]
pub struct PanelComponentProps {
    tabs: Vec<String>,
    current_tab: usize,
}

#[derive(Clone, Debug)]
pub struct PanelComponentState {
    side: Option<PanelSide>,
}

impl Default for PanelComponentState {
    fn default() -> Self {
        PanelComponentState { side: None }
    }
}

pub struct PanelStyle {
    boarder_color: Color,
    active_tab_bg: Color,
    active_tab_fg: Color,
}

pub struct PanelComponent {
    base: ComponentBase<PanelComponentProps, PanelComponentState>,
    tab: TabComponent,
}

impl PanelComponent {
    pub fn new(props: PanelComponentProps, state: PanelComponentState, tab: TabComponent) -> Self {
        PanelComponent {
            base: ComponentBase::new(Some(props), Some(state)),
            tab,
        }
    }

    pub fn empty() -> Self {
        PanelComponent {
            base: ComponentBase::new(None, None),
            tab: TabComponent::empty(),
        }
    }

    pub fn with_panel_state(panel_state: PanelState, side: PanelSide) -> Self {
        let tabs: Vec<String> = panel_state
            .tabs
            .iter()
            .map(|tab| tab.name.clone())
            .collect();
        let has_displayed_tabs = tabs.is_empty() == false;
        let panel_props = PanelComponentProps {
            tabs,
            current_tab: panel_state.current_tab,
        };

        let state = PanelComponentState {
            side: Some(side.clone()),
        };

        let tab = TabComponent::new(
            Some(TabComponentProps::new(
                panel_state.tabs[panel_state.current_tab].clone(),
                has_displayed_tabs,
                panel_state.is_focused,
                side,
            )),
            None,
        );

        PanelComponent::new(panel_props, state, tab)
    }
}

impl Component<Event, AppState, FileManagerActions> for PanelComponent {
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        if let Event::Keyboard(key_evt) = event {}

        self.tab.handle_event(event, store)
    }

    fn render<TBackend: Backend>(&self, frame: &mut Frame<TBackend>, area: Option<Rect>) {
        let props = self.base.get_props().unwrap();
        if props.tabs.len() > 1 {
            let tabs_items: Vec<Spans> = props
                .tabs
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    if idx == props.current_tab {
                        Spans::from(vec![Span::styled(val, Style::default().fg(Color::Blue))])
                    } else {
                        Spans::from(vec![Span::styled(val, Style::default())])
                    }
                })
                .collect();

            let tabs = Tabs::new(tabs_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(props.tabs[props.current_tab].clone()),
            );

            let layout = Layout::default()
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(area.unwrap());

            frame.render_widget(tabs, layout[0]);
            self.tab.render(frame, Some(layout[1]));
        } else {
            self.tab.render(frame, area);
        }
    }
}
