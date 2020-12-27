use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::List,
    widgets::ListItem,
    widgets::{Block, Borders},
};

use crate::{
    app::{
        actions::{
            AppAction, DirectoryAction, FileAction, FileManagerActions, PanelInfo, PanelSide,
            TabAction,
        },
        file_system::FileSystemItem,
        state::{AppState, ModalType, TabState},
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

pub struct TabStyle {
    active_border_color: Color,
    selected_element_background: Color,
    selected_element_foregound: Color,
    selected_element_indicator: String,
}

impl Default for TabStyle {
    fn default() -> Self {
        TabStyle {
            active_border_color: Color::Blue,
            selected_element_background: Color::Red,
            selected_element_foregound: Color::Black,
            selected_element_indicator: ">>".to_string(),
        }
    }
}

pub struct TabComponent {
    base: ComponentBase<TabComponentProps, ()>,
    style: TabStyle,
}

impl TabComponent {
    pub fn new(props: Option<TabComponentProps>, style: Option<TabStyle>) -> Self {
        TabComponent {
            base: ComponentBase::new(props, None),
            style: style.unwrap_or(TabStyle::default()),
        }
    }

    pub fn empty() -> Self {
        TabComponent::new(None, None)
    }

    fn current_item(&self) -> Option<FileSystemItem> {
        let props = self.base.get_props().unwrap();
        let state = props.state.unwrap();
        match state.tab_state.selected() {
            Some(idx) => Some(state.items[idx].clone()),
            None => None,
        }
    }
}

impl Component<Event, AppState, FileManagerActions> for TabComponent {
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        let props = self.base.get_props().unwrap();
        let tab_side = props.panel_side.unwrap();
        let tab_idx = match tab_side {
            PanelSide::Left => state.left_panel.current_tab,
            PanelSide::Right => state.right_panel.current_tab,
        };

        if let Event::Keyboard(key_evt) = event {
            if state.config.keyboard_cfg.move_down.is_pressed(key_evt) {
                store.dispatch(FileManagerActions::Tab(TabAction::Next));
                return true;
            }

            if state.config.keyboard_cfg.move_up.is_pressed(key_evt) {
                store.dispatch(FileManagerActions::Tab(TabAction::Previous));
                return true;
            }

            if state.config.keyboard_cfg.navigate_up.is_pressed(key_evt) && props.is_focused {
                let current_path = props.state.unwrap().path;
                if let Some(parent) = current_path.parent() {
                    store.dispatch(FileManagerActions::Directory(DirectoryAction::Open {
                        panel: PanelInfo {
                            path: parent.into(),
                            tab: tab_idx,
                            side: tab_side.clone(),
                        },
                        in_new_tab: false,
                    }));
                }

                return true;
            }

            if let Some(current_item) = self.current_item() {
                if state.config.keyboard_cfg.open_as_tab.is_pressed(key_evt) && props.is_focused {
                    if let FileSystemItem::Directory(dir) = current_item {
                        store.dispatch(FileManagerActions::Directory(DirectoryAction::Open {
                            panel: PanelInfo {
                                path: dir.get_path(),
                                tab: tab_idx,
                                side: tab_side.clone(),
                            },
                            in_new_tab: true,
                        }));
                    }

                    return true;
                }

                if state.config.keyboard_cfg.open.is_pressed(key_evt) && props.is_focused {
                    match current_item {
                        FileSystemItem::Directory(dir) => {
                            store.dispatch(FileManagerActions::Directory(DirectoryAction::Open {
                                panel: PanelInfo {
                                    path: dir.get_path(),
                                    tab: tab_idx,
                                    side: tab_side.clone(),
                                },
                                in_new_tab: false,
                            }));
                        }
                        FileSystemItem::File(file) => {
                            store.dispatch(FileManagerActions::File(FileAction::Open {
                                panel: PanelInfo {
                                    path: file.get_path(),
                                    tab: tab_idx,
                                    side: tab_side.clone(),
                                },
                            }))
                        }
                        _ => {}
                    };

                    return true;
                }

                if state.config.keyboard_cfg.delete.is_pressed(key_evt) && props.is_focused {
                    match current_item {
                        FileSystemItem::Directory(dir) => {
                            store.dispatch(FileManagerActions::Directory(
                                DirectoryAction::Delete {
                                    panel: PanelInfo {
                                        path: dir.get_path(),
                                        tab: tab_idx,
                                        side: tab_side.clone(),
                                    },
                                },
                            ));
                        }
                        FileSystemItem::File(file) => {
                            store.dispatch(FileManagerActions::File(FileAction::Delete {
                                panel: PanelInfo {
                                    path: file.get_path(),
                                    tab: tab_idx,
                                    side: tab_side.clone(),
                                },
                            }))
                        }
                        _ => {}
                    };

                    return true;
                }

                if state.config.keyboard_cfg.move_left.is_pressed(key_evt)
                    && props.is_focused
                    && tab_side == PanelSide::Right
                {
                    match current_item {
                        FileSystemItem::Directory(dir) => {
                            let name = dir.get_name();
                            let mut to_path = state.left_panel.tabs[state.left_panel.current_tab]
                                .path
                                .clone();
                            to_path.push(name);
                            store.dispatch(FileManagerActions::Directory(DirectoryAction::Move {
                                from: PanelInfo {
                                    path: dir.get_path(),
                                    tab: state.right_panel.current_tab,
                                    side: PanelSide::Right,
                                },
                                to: PanelInfo {
                                    path: to_path,
                                    tab: state.left_panel.current_tab,
                                    side: PanelSide::Left,
                                },
                            }));
                        }
                        FileSystemItem::File(file) => {
                            let name = file.get_name();
                            let mut to_path = state.left_panel.tabs[state.left_panel.current_tab]
                                .path
                                .clone();
                            to_path.push(name);
                            store.dispatch(FileManagerActions::File(FileAction::Move {
                                from: PanelInfo {
                                    path: file.get_path(),
                                    tab: state.right_panel.current_tab,
                                    side: PanelSide::Right,
                                },
                                to: PanelInfo {
                                    path: to_path,
                                    tab: state.left_panel.current_tab,
                                    side: PanelSide::Left,
                                },
                            }));
                        }
                        _ => {}
                    };

                    return true;
                }

                if state.config.keyboard_cfg.move_right.is_pressed(key_evt)
                    && props.is_focused
                    && tab_side == PanelSide::Left
                {
                    match current_item {
                        FileSystemItem::Directory(dir) => {
                            let name = dir.get_name();
                            let mut to_path = state.right_panel.tabs[state.right_panel.current_tab]
                                .path
                                .clone();
                            to_path.push(name);
                            store.dispatch(FileManagerActions::Directory(DirectoryAction::Move {
                                from: PanelInfo {
                                    path: dir.get_path(),
                                    tab: state.left_panel.current_tab,
                                    side: PanelSide::Left,
                                },
                                to: PanelInfo {
                                    path: to_path,
                                    tab: state.right_panel.current_tab,
                                    side: PanelSide::Right,
                                },
                            }));
                        }
                        FileSystemItem::File(file) => {
                            let name = file.get_name();
                            let mut to_path = state.right_panel.tabs[state.right_panel.current_tab]
                                .path
                                .clone();
                            to_path.push(name);
                            store.dispatch(FileManagerActions::File(FileAction::Move {
                                from: PanelInfo {
                                    path: file.get_path(),
                                    tab: state.left_panel.current_tab,
                                    side: PanelSide::Left,
                                },
                                to: PanelInfo {
                                    path: to_path,
                                    tab: state.right_panel.current_tab,
                                    side: PanelSide::Right,
                                },
                            }));
                        }
                        _ => {}
                    };

                    return true;
                }

                if state.config.keyboard_cfg.create.is_pressed(key_evt) && props.is_focused {
                    let tab_idx = match tab_side {
                        PanelSide::Left => state.left_panel.current_tab,
                        PanelSide::Right => state.right_panel.current_tab,
                    };
                    store.dispatch(FileManagerActions::App(AppAction::ShowModal(
                        ModalType::CreateModal {
                            panel_side: tab_side,
                            panel_tab: tab_idx,
                        },
                    )));
                    return true;
                }
            }
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

                let border_style = if tab_props.is_focused {
                    Style::default().fg(self.style.active_border_color)
                } else {
                    Style::default()
                };

                let block = Block::default()
                    .title(Spans::from(vec![
                        Span::from("| "),
                        Span::from(state.icon),
                        Span::from(" "),
                        Span::from(state.name),
                        Span::from(" |"),
                    ]))
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .border_type(tui::widgets::BorderType::Thick)
                    .style(Style::default());

                let list = List::new(list_items).block(block);

                if tab_props.is_focused {
                    let focused_list = List::from(list)
                        .highlight_style(
                            Style::default()
                                .bg(self.style.selected_element_background)
                                .fg(self.style.selected_element_foregound),
                        )
                        .highlight_symbol(self.style.selected_element_indicator.as_str());

                    frame.render_stateful_widget(focused_list, area.unwrap(), &mut state.tab_state);
                } else {
                    frame.render_widget(list, area.unwrap());
                }
            }
        }
    }
}
