use std::fmt::Debug;
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
        actions::{FileManagerActions, PanelAction, PanelSide},
        config::icon_cfg::IconsConfig,
        file_system::FileSystem,
        state::{AppState, PanelState},
    },
    core::{
        config::CoreConfig,
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
    },
};

use super::tab::{TabComponent, TabComponentProps};

#[derive(Clone, Default, Debug)]
pub struct PanelComponentProps {
    tabs: Vec<TabInfo>,
    current_tab: usize,
    is_focused: bool,
    show_icons: bool,
    tab_search: bool,
}

#[derive(Clone, Default, Debug)]
struct TabInfo {
    pub name: String,
    pub icon: String,
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
    active_border_color: Color,
    active_tab_bg: Color,
    active_tab_fg: Color,
}

impl Default for PanelStyle {
    fn default() -> Self {
        PanelStyle {
            active_border_color: Color::Blue,
            active_tab_bg: Color::Red,
            active_tab_fg: Color::Black,
        }
    }
}

pub struct PanelComponent<TFileSystem: Clone + Default + Debug + FileSystem> {
    base: ComponentBase<PanelComponentProps, PanelComponentState>,
    tab: TabComponent<TFileSystem>,
    style: PanelStyle,
    _marker: std::marker::PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Default + Debug + FileSystem> PanelComponent<TFileSystem> {
    pub fn new(
        props: PanelComponentProps,
        state: PanelComponentState,
        tab: TabComponent<TFileSystem>,
    ) -> Self {
        PanelComponent {
            base: ComponentBase::new(Some(props), Some(state)),
            tab,
            style: PanelStyle::default(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn empty() -> Self {
        PanelComponent {
            base: ComponentBase::new(None, None),
            tab: TabComponent::empty(),
            style: PanelStyle::default(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_panel_state(
        panel_state: PanelState<TFileSystem>,
        side: PanelSide,
        icons: &IconsConfig,
        core: &CoreConfig,
    ) -> Self {
        let tabs: Vec<_> = panel_state
            .tabs
            .iter()
            .map(|tab| TabInfo {
                name: tab.name.clone(),
                icon: icons.get_dir_icon(tab.name.clone()),
            })
            .collect();
        let tab_state = panel_state.tabs[panel_state.current_tab].clone();
        let has_displayed_tabs = tabs.is_empty() == false;
        let panel_props = PanelComponentProps {
            tabs,
            current_tab: panel_state.current_tab,
            is_focused: panel_state.is_focused,
            show_icons: icons.use_icons,
            tab_search: tab_state.search_mode,
        };

        let state = PanelComponentState {
            side: Some(side.clone()),
        };

        let tab = TabComponent::new(
            Some(TabComponentProps::new(
                tab_state,
                has_displayed_tabs,
                panel_state.is_focused,
                side,
                icons.use_icons,
                core.list_arrow.clone(),
            )),
            None,
        );

        PanelComponent::new(panel_props, state, tab)
    }

    pub fn tab_in_search_mode(&self) -> bool {
        self.base.get_props().unwrap().tab_search
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    Component<Event, AppState<TFileSystem>, FileManagerActions> for PanelComponent<TFileSystem>
{
    fn on_tick(&mut self, store: &mut Store<AppState<TFileSystem>, FileManagerActions>) {
        self.tab.on_tick(store);
    }

    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        let props = self.base.get_props().unwrap();
        let panel_side = self.base.get_state().unwrap().side.unwrap();
        if props.tab_search == false {
            if let Event::Keyboard(key_evt) = event {
                if state.config.keyboard_cfg.next_tab.is_pressed(key_evt)
                    && props.is_focused
                    && props.tabs.len() > 1
                {
                    store.dispatch(FileManagerActions::Panel(PanelAction::Next {
                        panel: panel_side,
                    }));
                    return true;
                }

                if state.config.keyboard_cfg.prev_tab.is_pressed(key_evt)
                    && props.is_focused
                    && props.tabs.len() > 1
                {
                    store.dispatch(FileManagerActions::Panel(PanelAction::Previous {
                        panel: panel_side,
                    }));
                    return true;
                }

                if state.config.keyboard_cfg.close.is_pressed(key_evt)
                    && props.is_focused
                    && props.tabs.len() > 1
                {
                    store.dispatch(FileManagerActions::Panel(PanelAction::CloseTab {
                        panel: panel_side,
                        tab: props.current_tab,
                    }));
                    return true;
                }
            }
        }

        self.tab.handle_event(event, store)
    }

    fn render<TBackend: Backend>(&self, frame: &mut Frame<TBackend>, area: Option<Rect>) {
        let props = self.base.get_props().unwrap();
        let show_icons = props.show_icons;
        if props.tabs.len() > 1 {
            let tabs_items: Vec<Spans> = props
                .tabs
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    if idx == props.current_tab {
                        let style = Style::default()
                            .fg(self.style.active_tab_fg)
                            .bg(self.style.active_tab_bg);
                        if show_icons {
                            Spans::from(vec![
                                Span::styled(val.icon.clone(), style),
                                Span::styled(" ", style),
                                Span::styled(val.name.clone(), style),
                            ])
                        } else {
                            Spans::from(vec![Span::styled(val.name.clone(), style)])
                        }
                    } else {
                        if show_icons {
                            Spans::from(vec![
                                Span::styled(val.icon.clone(), Style::default()),
                                Span::styled(" ", Style::default()),
                                Span::styled(val.name.clone(), Style::default()),
                            ])
                        } else {
                            Spans::from(vec![Span::styled(val.name.clone(), Style::default())])
                        }
                    }
                })
                .collect();

            let style = Style::default();
            if props.is_focused {
                style.fg(self.style.active_border_color);
            }

            let tabs =
                Tabs::new(tabs_items).block(Block::default().style(style).borders(Borders::ALL));

            let layout = Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(area.unwrap());

            frame.render_widget(tabs, layout[0]);
            self.tab.render(frame, Some(layout[1]));
        } else {
            self.tab.render(frame, area);
        }
    }
}
