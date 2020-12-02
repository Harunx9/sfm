use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders},
};

use crate::core::{
    events::Event,
    store::Store,
    ui::{Component, ComponentBase},
};

use super::{
    actions::{AppActions, FileManagerActions},
    state::{AppState, TabState},
};

pub struct RootComponent {
    left_tab: TabComponent,
    right_tab: TabComponent,
}

impl RootComponent {
    pub fn new() -> Self {
        RootComponent {
            left_tab: TabComponent::empty(),
            right_tab: TabComponent::empty(),
        }
    }
}

impl Component<Event, (), AppState, FileManagerActions> for RootComponent {
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState, FileManagerActions>,
    ) -> bool {
        let mut result = false;
        if let Event::Keyboard(key_evt) = event {
            if let KeyCode::Char('q') = key_evt.code {
                store.dispatch(FileManagerActions::App(AppActions::Exit));
                result = true
            }
        }

        result = self.left_tab.handle_event(event, store);
        if result == true {
            return result;
        }
        result = self.right_tab.handle_event(event, store);

        result
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>, area: Option<Rect>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());
        self.left_tab.render(frame, Some(layout[0]));
        self.right_tab.render(frame, Some(layout[1]));
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TabProps {
    current_selected: u32,
    max_lenght: u32,
}

pub struct TabComponent {
    base: ComponentBase<TabProps, TabState>,
}

impl TabComponent {
    pub fn new(props: Option<TabProps>, state: Option<TabState>) -> Self {
        TabComponent {
            base: ComponentBase::new(props, state),
        }
    }

    pub fn with_props(props: TabProps) -> Self {
        TabComponent::new(Some(props), None)
    }

    pub fn empty() -> Self {
        TabComponent::new(None, None)
    }
}

impl Component<Event, TabState, AppState, FileManagerActions> for TabComponent {
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
