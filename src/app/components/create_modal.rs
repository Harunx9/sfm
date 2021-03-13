use std::{fmt::Debug, path::PathBuf};

use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

use crate::{
    app::{
        actions::{
            AppAction, DirectoryAction, FileAction, FileManagerActions, PanelInfo, PanelSide,
            SymlinkAction,
        },
        file_system::FileSystem,
        state::{AppState, TabIdx},
    },
    core::{
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
    },
};

use super::{create_modal_layout, ModalStyle};

#[derive(Clone, Default)]
pub struct CreateModalProps {
    panel_side: Option<PanelSide>,
    item_to_symlink: Option<usize>,
    panel_tab: TabIdx,
    dir_path: PathBuf,
    show_icons: bool,
    file_icon: String,
    dir_icon: String,
    symlink_icon: String,
    list_selector: String,
    modal_style: ModalStyle,
}

impl CreateModalProps {
    pub fn new(
        panel_side: PanelSide,
        panel_tab: TabIdx,
        dir_path: PathBuf,
        item_to_symlink: Option<usize>,
        show_icons: bool,
        file_icon: String,
        dir_icon: String,
        symlink_icon: String,
        list_selector: String,
        modal_style: ModalStyle,
    ) -> Self {
        Self {
            item_to_symlink,
            panel_side: Some(panel_side),
            panel_tab,
            dir_path,
            show_icons,
            file_icon,
            dir_icon,
            symlink_icon,
            list_selector,
            modal_style,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CreateOption {
    File,
    Dir,
    Symlink,
}

impl ToString for CreateOption {
    fn to_string(&self) -> String {
        match self {
            CreateOption::File => "File".to_string(),
            CreateOption::Dir => "Directory".to_string(),
            CreateOption::Symlink => "Symlink".to_string(),
        }
    }
}

impl From<String> for CreateOption {
    fn from(source: String) -> Self {
        match source.as_str() {
            "File" => CreateOption::File,
            "Directory" => CreateOption::Dir,
            "Symlink" => CreateOption::Symlink,
            _ => panic!(""),
        }
    }
}

impl From<usize> for CreateOption {
    fn from(source: usize) -> Self {
        match source {
            0 => CreateOption::File,
            1 => CreateOption::Dir,
            2 => CreateOption::Symlink,
            _ => panic!(""),
        }
    }
}

#[derive(Clone, Default)]
pub struct CreateModalState {
    create_selection: Option<CreateOption>,
    input: String,
    list_state: ListState,
}

pub struct CreateModalComponent<TFileSystem: Clone + Debug + Default + FileSystem> {
    base: ComponentBase<CreateModalProps, CreateModalState>,
    _maker: std::marker::PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> CreateModalComponent<TFileSystem> {
    pub fn with_props(props: CreateModalProps) -> Self {
        CreateModalComponent {
            base: ComponentBase::new(Some(props), Some(CreateModalState::default())),
            _maker: std::marker::PhantomData,
        }
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    Component<Event, AppState<TFileSystem>, FileManagerActions>
    for CreateModalComponent<TFileSystem>
{
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        let local_state = self.base.get_state().unwrap();
        let props = self.base.get_props().unwrap();
        if let Event::Keyboard(key_evt) = event {
            if local_state.create_selection.is_none() {
                if state.config.keyboard_cfg.move_up.is_pressed(key_evt) {
                    let next_item = match local_state.list_state.selected() {
                        Some(current) => {
                            if current == 0 {
                                2
                            } else {
                                current - 1
                            }
                        }
                        None => 0,
                    };
                    self.base.set_state(|mut current_state| {
                        current_state.list_state.select(Some(next_item));
                        CreateModalState {
                            list_state: current_state.list_state,
                            ..current_state
                        }
                    });
                    return true;
                }

                if state.config.keyboard_cfg.move_down.is_pressed(key_evt) {
                    let next_item = match local_state.list_state.selected() {
                        Some(current) => {
                            if current >= 2 {
                                0
                            } else {
                                current + 1
                            }
                        }
                        None => 0,
                    };
                    self.base.set_state(|mut current_state| {
                        current_state.list_state.select(Some(next_item));
                        CreateModalState {
                            list_state: current_state.list_state,
                            ..current_state
                        }
                    });
                    return true;
                }

                if state.config.keyboard_cfg.accept.is_pressed(key_evt) {
                    self.base.set_state(|current_state| {
                        let create_selection =
                            CreateOption::from(current_state.list_state.selected().unwrap_or(0));
                        CreateModalState {
                            create_selection: Some(create_selection),
                            ..current_state
                        }
                    });
                }
            } else if let Some(create_selection) = local_state.create_selection {
                if state.config.keyboard_cfg.accept.is_pressed(key_evt)
                    && local_state.input.is_empty() == false
                {
                    let panel_side = props.panel_side.unwrap();
                    match create_selection {
                        CreateOption::File => {
                            store.dispatch(FileManagerActions::File(FileAction::Create {
                                file_name: local_state.input.clone(),
                                panel: PanelInfo {
                                    side: panel_side,
                                    tab: props.panel_tab,
                                    path: props.dir_path,
                                },
                            }))
                        }
                        CreateOption::Dir => {
                            store.dispatch(FileManagerActions::Directory(DirectoryAction::Create {
                                dir_name: local_state.input.clone(),
                                panel: PanelInfo {
                                    side: panel_side,
                                    tab: props.panel_tab,
                                    path: props.dir_path,
                                },
                            }))
                        }
                        CreateOption::Symlink => {
                            let item_path = match props.panel_side.unwrap() {
                                PanelSide::Left => state.left_panel.tabs[props.panel_tab].items
                                    [props.item_to_symlink.unwrap()]
                                .get_path(),
                                PanelSide::Right => state.right_panel.tabs[props.panel_tab].items
                                    [props.item_to_symlink.unwrap()]
                                .get_path(),
                            };

                            store.dispatch(FileManagerActions::Symlink(SymlinkAction::Create {
                                symlink_path: PathBuf::from(local_state.input.clone()),
                                panel: PanelInfo {
                                    path: item_path,
                                    side: panel_side,
                                    tab: props.panel_tab,
                                },
                            }))
                        }
                    };

                    store.dispatch(FileManagerActions::App(AppAction::CloseModal));
                    return true;
                }

                match key_evt.code {
                    KeyCode::Char(c) => {
                        self.base.set_state(|current_state| {
                            let mut current_text = current_state.input.clone();
                            if key_evt.modifiers == KeyModifiers::SHIFT {
                                current_text =
                                    format!("{}{}", current_text, c.to_uppercase().to_string());
                            } else {
                                current_text.push(c);
                            }

                            CreateModalState {
                                input: current_text,
                                ..current_state
                            }
                        });
                        return true;
                    }
                    KeyCode::Backspace => {
                        self.base.set_state(|current_state| {
                            let mut current_text = current_state.input.clone();
                            current_text.pop();

                            CreateModalState {
                                input: current_text,
                                ..current_state
                            }
                        });
                        return true;
                    }
                    _ => {}
                };
            }

            if state.config.keyboard_cfg.close.is_pressed(key_evt) {
                store.dispatch(FileManagerActions::App(AppAction::CloseModal));
                return true;
            }
        }
        false
    }

    fn render<TBackend: tui::backend::Backend>(
        &self,
        frame: &mut tui::Frame<TBackend>,
        area: Option<tui::layout::Rect>,
    ) {
        let layout = if let Some(area) = area {
            create_modal_layout(50, 10, area)
        } else {
            create_modal_layout(50, 10, frame.size())
        };

        let mut local_state = self.base.get_state().unwrap();
        let props = self.base.get_props().unwrap();

        if let Some(create_selection) = local_state.create_selection {
            let block = Block::default()
                .title(Spans::from(vec![
                    Span::from("| "),
                    if create_selection == CreateOption::Symlink {
                        Span::from("Symlink path:")
                    } else {
                        Span::from("Item name:")
                    },
                    Span::from(" |"),
                ]))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(props.modal_style.border_color))
                .border_type(tui::widgets::BorderType::Thick)
                .style(Style::default());

            let paragraph = Paragraph::new(local_state.input)
                .block(block)
                .alignment(tui::layout::Alignment::Center);

            frame.render_widget(Clear, layout);
            frame.render_widget(paragraph, layout);
        } else {
            let mut items = if props.show_icons {
                vec![
                    ListItem::new(Spans::from(vec![
                        Span::from(props.file_icon),
                        Span::from(" "),
                        Span::from(CreateOption::File.to_string()),
                    ])),
                    ListItem::new(Spans::from(vec![
                        Span::from(props.dir_icon),
                        Span::from(" "),
                        Span::from(CreateOption::Dir.to_string()),
                    ])),
                ]
            } else {
                vec![
                    ListItem::new(Spans::from(vec![Span::from(
                        CreateOption::File.to_string(),
                    )])),
                    ListItem::new(Spans::from(vec![Span::from(CreateOption::Dir.to_string())])),
                ]
            };

            if props.item_to_symlink.is_some() {
                if props.show_icons {
                    items.push(ListItem::new(Spans::from(vec![
                        Span::from(props.symlink_icon),
                        Span::from(" "),
                        Span::from(CreateOption::Symlink.to_string()),
                    ])));
                } else {
                    items.push(ListItem::new(Spans::from(vec![Span::from(
                        CreateOption::Symlink.to_string(),
                    )])));
                }
            }

            let block = Block::default()
                .title(Spans::from(vec![
                    Span::from("| "),
                    Span::from("Chose item to create:"),
                    Span::from(" |"),
                ]))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(props.modal_style.border_color))
                .border_type(tui::widgets::BorderType::Thick)
                .style(Style::default().bg(tui::style::Color::Reset));

            let list = List::new(items)
                .block(block)
                .highlight_style(
                    Style::default()
                        .fg(props.modal_style.selected_element_foreground)
                        .bg(props.modal_style.selected_element_background),
                )
                .highlight_symbol(props.list_selector.as_str());

            frame.render_widget(Clear, layout);
            frame.render_stateful_widget(list, layout, &mut local_state.list_state);
        }
    }
}
