use crossterm::event::KeyCode;
use std::fmt::Debug;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::List,
    widgets::ListItem,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::{
        actions::{
            AppAction, DirectoryAction, FileAction, FileManagerActions, PanelInfo, PanelSide,
            SearchAction, SymlinkAction, TabAction,
        },
        file_system::{file_system_item::FileSystemItem, FileSystem},
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
pub struct TabComponentProps<TFileSystem: Clone + Debug + Default + FileSystem> {
    state: Option<TabState<TFileSystem>>,
    has_displayed_tabs: bool,
    is_focused: bool,
    panel_side: Option<PanelSide>,
    show_icons: bool,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> Default for TabComponentProps<TFileSystem> {
    fn default() -> Self {
        TabComponentProps {
            state: None,
            has_displayed_tabs: false,
            is_focused: false,
            panel_side: None,
            show_icons: false,
        }
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> TabComponentProps<TFileSystem> {
    pub fn new(
        state: TabState<TFileSystem>,
        has_displayed_tabs: bool,
        is_focused: bool,
        panel_side: PanelSide,
        show_icons: bool,
    ) -> Self {
        TabComponentProps {
            state: Some(state),
            has_displayed_tabs,
            is_focused,
            panel_side: Some(panel_side),
            show_icons,
        }
    }
}

pub struct TabStyle {
    active_border_color: Color,
    selected_element_background: Color,
    selected_element_foreground: Color,
    selected_element_indicator: String,
}

impl Default for TabStyle {
    fn default() -> Self {
        TabStyle {
            active_border_color: Color::Blue,
            selected_element_background: Color::Red,
            selected_element_foreground: Color::Black,
            selected_element_indicator: ">>".to_string(),
        }
    }
}

pub struct TabComponent<TFileSystem: Clone + Debug + Default + FileSystem> {
    base: ComponentBase<TabComponentProps<TFileSystem>, ()>,
    style: TabStyle,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> TabComponent<TFileSystem> {
    pub fn new(props: Option<TabComponentProps<TFileSystem>>, style: Option<TabStyle>) -> Self {
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
        let items = state.filtered_items();
        match state.tab_state.selected() {
            Some(idx) => Some(items[idx].clone()),
            None => None,
        }
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    Component<Event, AppState<TFileSystem>, FileManagerActions> for TabComponent<TFileSystem>
{
    fn on_tick(&mut self, store: &mut Store<AppState<TFileSystem>, FileManagerActions>) {
        let props = self.base.get_props().unwrap();
        let local_state = props.state.unwrap();
        let global_state = store.get_state();

        for item in local_state.items.iter() {
            if global_state.file_system.exist(item.get_path().as_path()) == false {
                store.dispatch(FileManagerActions::Tab(TabAction::ReloadTab {
                    panel_side: props.panel_side.unwrap(),
                    path: local_state.path.clone(),
                }));
                return;
            }
        }
    }

    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        let props = self.base.get_props().unwrap();
        let tab_side = props.panel_side.unwrap();
        let tab_idx = match tab_side {
            PanelSide::Left => state.left_panel.current_tab,
            PanelSide::Right => state.right_panel.current_tab,
        };
        let tab_state = match tab_side {
            PanelSide::Left => state.left_panel.tabs[state.left_panel.current_tab].clone(),
            PanelSide::Right => state.right_panel.tabs[state.right_panel.current_tab].clone(),
        };
        if props.is_focused {
            if let Event::Keyboard(key_evt) = event {
                if state.config.keyboard_cfg.close.is_pressed(key_evt)
                    && (tab_state.search_mode || tab_state.phrase.is_empty() == false)
                {
                    store.dispatch(FileManagerActions::Search(SearchAction::Stop {
                        tab: tab_idx,
                        panel_side: tab_side,
                    }));
                    return true;
                }

                if state.config.keyboard_cfg.accept.is_pressed(key_evt)
                    && (tab_state.search_mode && tab_state.phrase.is_empty() == false)
                {
                    store.dispatch(FileManagerActions::Search(SearchAction::ApplySearch {
                        tab: tab_idx,
                        panel_side: tab_side,
                    }));
                    return true;
                }

                if tab_state.search_mode {
                    let mut phrase = tab_state.phrase;
                    match key_evt.code {
                        KeyCode::Char(c) => {
                            phrase.push(c);
                        }
                        KeyCode::Backspace => {
                            phrase.pop();
                        }
                        _ => {}
                    };
                    store.dispatch(FileManagerActions::Search(SearchAction::Input {
                        tab: tab_idx,
                        panel_side: tab_side,
                        phrase,
                    }));
                    return true;
                }

                if state
                    .config
                    .keyboard_cfg
                    .search_in_panel
                    .is_pressed(key_evt)
                    && tab_state.search_mode == false
                {
                    store.dispatch(FileManagerActions::Search(SearchAction::Start {
                        tab: tab_idx,
                        panel_side: tab_side,
                    }));
                    return true;
                }

                if state.config.keyboard_cfg.move_down.is_pressed(key_evt) {
                    store.dispatch(FileManagerActions::Tab(TabAction::Next));
                    return true;
                }

                if state.config.keyboard_cfg.move_up.is_pressed(key_evt) {
                    store.dispatch(FileManagerActions::Tab(TabAction::Previous));
                    return true;
                }

                if state.config.keyboard_cfg.select_next.is_pressed(key_evt) {
                    store.dispatch(FileManagerActions::Tab(TabAction::SelectNext));
                    return true;
                }

                if state.config.keyboard_cfg.select_prev.is_pressed(key_evt) {
                    store.dispatch(FileManagerActions::Tab(TabAction::SelectPrev));
                    return true;
                }

                if state.config.keyboard_cfg.close.is_pressed(key_evt)
                    && tab_state.selected.is_empty() == false
                {
                    store.dispatch(FileManagerActions::Tab(TabAction::ClearSelection));
                    return true;
                }

                if state.config.keyboard_cfg.navigate_up.is_pressed(key_evt) && props.is_focused {
                    let current_path = tab_state.path;
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
                if state.config.keyboard_cfg.open_as_tab.is_pressed(key_evt) && props.is_focused {
                    for item in tab_state.selected.iter() {
                        match item {
                            FileSystemItem::Directory(dir) => {
                                store.dispatch(FileManagerActions::Directory(
                                    DirectoryAction::Open {
                                        panel: PanelInfo {
                                            path: dir.get_path(),
                                            tab: tab_idx,
                                            side: tab_side.clone(),
                                        },
                                        in_new_tab: true,
                                    },
                                ));
                            }
                            FileSystemItem::Symlink(symlink) => {
                                store.dispatch(FileManagerActions::Symlink(SymlinkAction::Open {
                                    panel: PanelInfo {
                                        path: symlink.get_path(),
                                        tab: tab_idx,
                                        side: tab_side.clone(),
                                    },
                                    in_new_tab: true,
                                }))
                            }
                            _ => {}
                        };
                    }
                    store.dispatch(FileManagerActions::Tab(TabAction::ClearSelection));
                    return true;
                }

                if state.config.keyboard_cfg.open.is_pressed(key_evt) && props.is_focused {
                    for item in tab_state.selected.iter() {
                        match item {
                            FileSystemItem::Directory(dir) => {
                                store.dispatch(FileManagerActions::Directory(
                                    DirectoryAction::Open {
                                        panel: PanelInfo {
                                            path: dir.get_path(),
                                            tab: tab_idx,
                                            side: tab_side.clone(),
                                        },
                                        in_new_tab: false,
                                    },
                                ));
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
                            FileSystemItem::Symlink(symlink) => {
                                store.dispatch(FileManagerActions::Symlink(SymlinkAction::Open {
                                    panel: PanelInfo {
                                        path: symlink.get_path(),
                                        tab: tab_idx,
                                        side: tab_side.clone(),
                                    },
                                    in_new_tab: false,
                                }))
                            }
                            _ => {}
                        };
                    }
                    store.dispatch(FileManagerActions::Tab(TabAction::ClearSelection));

                    return true;
                }

                if state.config.keyboard_cfg.delete.is_pressed(key_evt) && props.is_focused {
                    for item in tab_state.selected.iter() {
                        match item {
                            FileSystemItem::Directory(dir) => {
                                store.dispatch(FileManagerActions::Directory(
                                    DirectoryAction::Delete {
                                        panel: PanelInfo {
                                            path: dir.get_path(),
                                            tab: tab_idx,
                                            side: tab_side.clone(),
                                        },
                                        is_empty: dir.is_empty(),
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
                            FileSystemItem::Symlink(symlink) => {
                                store.dispatch(FileManagerActions::Symlink(SymlinkAction::Delete {
                                    panel: PanelInfo {
                                        path: symlink.get_path(),
                                        tab: tab_idx,
                                        side: tab_side.clone(),
                                    },
                                }))
                            }
                            _ => {}
                        };
                    }
                    store.dispatch(FileManagerActions::Tab(TabAction::ClearSelection));

                    return true;
                }

                if state.config.keyboard_cfg.move_left.is_pressed(key_evt)
                    && props.is_focused
                    && tab_side == PanelSide::Right
                {
                    for item in tab_state.selected.iter() {
                        match item {
                            FileSystemItem::Directory(dir) => {
                                let name = dir.get_name();
                                let mut to_path = state.left_panel.tabs
                                    [state.left_panel.current_tab]
                                    .path
                                    .clone();
                                if dir.get_path() == to_path {
                                    store.dispatch(FileManagerActions::App(AppAction::ShowModal(
                                        ModalType::ErrorModal(format!(
                                            "Can't move \n {} \n into \n {}",
                                            dir.get_path().to_str().unwrap_or(""),
                                            to_path.to_str().unwrap_or("")
                                        )),
                                    )));
                                } else {
                                    to_path.push(name);
                                    store.dispatch(FileManagerActions::Directory(
                                        DirectoryAction::Move {
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
                                        },
                                    ));
                                }
                            }
                            FileSystemItem::File(file) => {
                                let name = file.get_name();
                                let mut to_path = state.left_panel.tabs
                                    [state.left_panel.current_tab]
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
                    }
                    store.dispatch(FileManagerActions::Tab(TabAction::ClearSelection));

                    return true;
                }

                if state.config.keyboard_cfg.move_right.is_pressed(key_evt)
                    && props.is_focused
                    && tab_side == PanelSide::Left
                {
                    for item in tab_state.selected.iter() {
                        match item {
                            FileSystemItem::Directory(dir) => {
                                let name = dir.get_name();
                                let mut to_path = state.right_panel.tabs
                                    [state.right_panel.current_tab]
                                    .path
                                    .clone();
                                if dir.get_path() == to_path {
                                    store.dispatch(FileManagerActions::App(AppAction::ShowModal(
                                        ModalType::ErrorModal(format!(
                                            "Can't move \n {} \n into \n {}",
                                            dir.get_path().to_str().unwrap_or(""),
                                            to_path.to_str().unwrap_or("")
                                        )),
                                    )));
                                } else {
                                    to_path.push(name);
                                    store.dispatch(FileManagerActions::Directory(
                                        DirectoryAction::Move {
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
                                        },
                                    ));
                                }
                            }
                            FileSystemItem::File(file) => {
                                let name = file.get_name();
                                let mut to_path = state.right_panel.tabs
                                    [state.right_panel.current_tab]
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
                    }
                    store.dispatch(FileManagerActions::Tab(TabAction::ClearSelection));

                    return true;
                }

                if state.config.keyboard_cfg.copy_to_left.is_pressed(key_evt)
                    && props.is_focused
                    && tab_side == PanelSide::Right
                {
                    for item in tab_state.selected.iter() {
                        match item {
                            FileSystemItem::Directory(dir) => {
                                let name = dir.get_name();
                                let mut to_path = state.left_panel.tabs
                                    [state.left_panel.current_tab]
                                    .path
                                    .clone();
                                to_path.push(name);
                                store.dispatch(FileManagerActions::Directory(
                                    DirectoryAction::Copy {
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
                                    },
                                ));
                            }
                            FileSystemItem::File(file) => {
                                let name = file.get_name();
                                let mut to_path = state.left_panel.tabs
                                    [state.left_panel.current_tab]
                                    .path
                                    .clone();
                                to_path.push(name);
                                store.dispatch(FileManagerActions::File(FileAction::Copy {
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
                    }
                    store.dispatch(FileManagerActions::Tab(TabAction::ClearSelection));

                    return true;
                }

                if state.config.keyboard_cfg.copy_to_right.is_pressed(key_evt)
                    && props.is_focused
                    && tab_side == PanelSide::Left
                {
                    for item in tab_state.selected.iter() {
                        match item {
                            FileSystemItem::Directory(dir) => {
                                let name = dir.get_name();
                                let mut to_path = state.right_panel.tabs
                                    [state.right_panel.current_tab]
                                    .path
                                    .clone();
                                to_path.push(name);
                                store.dispatch(FileManagerActions::Directory(
                                    DirectoryAction::Copy {
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
                                    },
                                ));
                            }
                            FileSystemItem::File(file) => {
                                let name = file.get_name();
                                let mut to_path = state.right_panel.tabs
                                    [state.right_panel.current_tab]
                                    .path
                                    .clone();
                                to_path.push(name);
                                store.dispatch(FileManagerActions::File(FileAction::Copy {
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
                    }
                    store.dispatch(FileManagerActions::Tab(TabAction::ClearSelection));

                    return true;
                }

                if tab_state.selected.len() == 1 || tab_state.tab_state.selected().is_none() {
                    if let Some(current_item) = self.current_item() {
                        if state.config.keyboard_cfg.rename.is_pressed(key_evt) && props.is_focused
                        {
                            let tab_idx = match tab_side {
                                PanelSide::Left => state.left_panel.current_tab,
                                PanelSide::Right => state.right_panel.current_tab,
                            };
                            store.dispatch(FileManagerActions::App(AppAction::ShowModal(
                                ModalType::RenameModal {
                                    panel_side: tab_side,
                                    panel_tab: tab_idx,
                                    item: current_item,
                                },
                            )));
                            return true;
                        }
                    }

                    if state.config.keyboard_cfg.create.is_pressed(key_evt) && props.is_focused {
                        let tab_idx = match tab_side {
                            PanelSide::Left => state.left_panel.current_tab,
                            PanelSide::Right => state.right_panel.current_tab,
                        };
                        store.dispatch(FileManagerActions::App(AppAction::ShowModal(
                            ModalType::CreateModal {
                                item_index: tab_state.tab_state.selected(),
                                panel_side: tab_side,
                                panel_tab: tab_idx,
                                panel_tab_path: tab_state.path.clone(),
                            },
                        )));
                        return true;
                    }
                }
            }
        }
        false
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>, area: Option<Rect>) {
        if let Some(tab_props) = self.base.get_props() {
            let show_icons = tab_props.show_icons;
            if let Some(mut state) = tab_props.state {
                let layout = if (state.search_mode || state.phrase.is_empty() == false)
                    && tab_props.is_focused
                {
                    Layout::default()
                        .constraints([Constraint::Min(0), Constraint::Length(3)])
                        .split(area.unwrap())
                } else {
                    vec![area.unwrap()]
                };

                let list_items: Vec<ListItem> = if state.phrase.is_empty() {
                    state
                        .items
                        .iter()
                        .map(|item| {
                            if state
                                .selected
                                .iter()
                                .any(|i| i.get_path() == item.get_path())
                            {
                                ListItem::new(
                                    item.to_spans(area.unwrap_or(frame.size()), show_icons),
                                )
                                .style(
                                    Style::default()
                                        .bg(self.style.selected_element_background)
                                        .fg(self.style.selected_element_foreground),
                                )
                            } else {
                                ListItem::new(
                                    item.to_spans(area.unwrap_or(frame.size()), show_icons),
                                )
                            }
                        })
                        .collect()
                } else {
                    state
                        .items
                        .iter()
                        .filter(|item| {
                            item.get_name()
                                .to_lowercase()
                                .contains(&state.phrase.to_lowercase())
                        })
                        .map(|item| {
                            if state
                                .selected
                                .iter()
                                .any(|i| i.get_path() == item.get_path())
                            {
                                ListItem::new(
                                    item.to_spans(area.unwrap_or(frame.size()), show_icons),
                                )
                                .style(
                                    Style::default()
                                        .bg(self.style.selected_element_background)
                                        .fg(self.style.selected_element_foreground),
                                )
                            } else {
                                ListItem::new(
                                    item.to_spans(area.unwrap_or(frame.size()), show_icons),
                                )
                            }
                        })
                        .collect()
                };

                let border_style = if tab_props.is_focused {
                    Style::default().fg(self.style.active_border_color)
                } else {
                    Style::default()
                };

                let block = Block::default()
                    .title(if show_icons {
                        Spans::from(vec![
                            Span::from("| "),
                            Span::from(state.icon),
                            Span::from(" "),
                            Span::from(state.name),
                            Span::from(" |"),
                        ])
                    } else {
                        Spans::from(vec![
                            Span::from("| "),
                            Span::from(state.name),
                            Span::from(" |"),
                        ])
                    })
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
                                .fg(self.style.selected_element_foreground),
                        )
                        .highlight_symbol(self.style.selected_element_indicator.as_str());

                    frame.render_stateful_widget(focused_list, layout[0], &mut state.tab_state);
                } else {
                    frame.render_widget(list, layout[0]);
                }

                if (state.search_mode || state.phrase.is_empty() == false) && tab_props.is_focused {
                    let block = Block::default()
                        .title(Spans::from(vec![
                            Span::from("| "),
                            Span::from("Search"),
                            Span::from(" |"),
                        ]))
                        .borders(Borders::ALL)
                        .border_style(border_style)
                        .border_type(tui::widgets::BorderType::Thick)
                        .style(Style::default());
                    let paragraph = Paragraph::new(format!("{}", state.phrase))
                        .block(block)
                        .alignment(tui::layout::Alignment::Left);
                    frame.render_widget(paragraph, layout[1]);
                }
            }
        }
    }
}
