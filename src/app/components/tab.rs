use tui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    widgets::List,
    widgets::ListItem,
    widgets::{Block, Borders},
};

use crate::{
    app::{
        actions::{FileManagerActions, PanelSide, TabAction},
        state::{AppState, TabState},
    },
    core::{
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
        ToSpans,
    },
};

#[derive(Clone, Debug)]
pub struct TabComponentProps {
    state: Option<TabState>,
    has_displayed_tabs: bool,
    is_focused: bool,
    panel_side: Option<PanelSide>,
}

impl Default for TabComponentProps {
    fn default() -> Self {
        TabComponentProps {
            state: None,
            has_displayed_tabs: false,
            is_focused: false,
            panel_side: None,
        }
    }
}

impl TabComponentProps {
    pub fn new(
        state: TabState,
        has_displayed_tabs: bool,
        is_focused: bool,
        panel_side: PanelSide,
    ) -> Self {
        TabComponentProps {
            state: Some(state),
            has_displayed_tabs,
            is_focused,
            panel_side: Some(panel_side),
        }
    }
}

pub struct TabComponent {
    base: ComponentBase<TabComponentProps, ()>,
}

impl TabComponent {
    pub fn new(props: Option<TabComponentProps>) -> Self {
        TabComponent {
            base: ComponentBase::new(props, None),
        }
    }

    pub fn empty() -> Self {
        TabComponent::new(None)
    }
}

impl Component<Event, AppState, FileManagerActions> for TabComponent {
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();

        if let Event::Keyboard(key_evt) = event {
            if state.config.keyboard_cfg.next_tab_item.is_pressed(key_evt) {
                store.dispatch(FileManagerActions::Tab(TabAction::Next));
                return true;
            }

            if state.config.keyboard_cfg.prev_tab_item.is_pressed(key_evt) {
                store.dispatch(FileManagerActions::Tab(TabAction::Previous));
                return true;
            }

            if state.config.keyboard_cfg.open.is_pressed(key_evt) {}
        }

        false
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>, area: Option<Rect>) {
        if let Some(tab_props) = self.base.get_props() {
            if let Some(mut state) = tab_props.state {
                let list_items: Vec<ListItem> = state
                    .items
                    .iter()
                    .map(|item| ListItem::new(item.to_spans(area.unwrap_or(frame.size()))))
                    .collect();

                let block = Block::default()
                    .title(state.name)
                    .borders(Borders::ALL)
                    .border_style(Style::default())
                    .border_type(tui::widgets::BorderType::Thick)
                    .style(Style::default());

                let list = List::new(list_items).block(block);

                if tab_props.is_focused {
                    let focused_list = List::from(list)
                        .highlight_style(Style::default())
                        .highlight_symbol(">>");

                    frame.render_stateful_widget(focused_list, area.unwrap(), &mut state.tab_state);
                } else {
                    frame.render_widget(list, area.unwrap());
                }
            }
        }
    }
}
