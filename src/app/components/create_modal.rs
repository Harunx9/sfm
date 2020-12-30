use std::path::PathBuf;

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
        },
        state::{AppState, TabIdx},
    },
    core::{
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
    },
};

use super::create_modal_layout;

#[derive(Clone, Default)]
pub struct CreateModalProps {
    panel_side: Option<PanelSide>,
    panel_tab: TabIdx,
    dir_path: PathBuf,
}

impl CreateModalProps {
    pub fn new(panel_side: PanelSide, panel_tab: TabIdx, dir_path: PathBuf) -> Self {
        Self {
            panel_side: Some(panel_side),
            panel_tab,
            dir_path,
        }
    }
}

#[derive(Clone, Copy)]
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

pub struct CreateModalComponent {
    base: ComponentBase<CreateModalProps, CreateModalState>,
}

impl CreateModalComponent {
    pub fn with_props(props: CreateModalProps) -> Self {
        CreateModalComponent {
            base: ComponentBase::new(Some(props), Some(CreateModalState::default())),
        }
    }
}

impl Component<Event, AppState, FileManagerActions> for CreateModalComponent {
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState, FileManagerActions>,
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
                    let panle_side = props.panel_side.unwrap();
                    match create_selection {
                        CreateOption::File => {
                            store.dispatch(FileManagerActions::File(FileAction::Create {
                                file_name: local_state.input.clone(),
                                panel: PanelInfo {
                                    side: panle_side,
                                    tab: props.panel_tab,
                                    path: props.dir_path,
                                },
                            }))
                        }
                        CreateOption::Dir => {
                            store.dispatch(FileManagerActions::Directory(DirectoryAction::Create {
                                dir_name: local_state.input.clone(),
                                panel: PanelInfo {
                                    side: panle_side,
                                    tab: props.panel_tab,
                                    path: props.dir_path,
                                },
                            }))
                        }
                        CreateOption::Symlink => {}
                    };

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
            create_modal_layout(50, 50, area)
        } else {
            create_modal_layout(50, 50, frame.size())
        };

        let mut local_state = self.base.get_state().unwrap();

        if local_state.create_selection.is_some() {
            let block = Block::default()
                .title(Spans::from(vec![
                    Span::from("| "),
                    Span::from("Chose item to create:"),
                    Span::from(" |"),
                ]))
                .borders(Borders::ALL)
                .border_style(Style::default())
                .border_type(tui::widgets::BorderType::Thick)
                .style(Style::default());

            let paragraph = Paragraph::new(local_state.input).block(block);
            frame.render_widget(paragraph, layout);
        } else {
            let items = vec![
                ListItem::new(CreateOption::File.to_string()),
                ListItem::new(CreateOption::Dir.to_string()),
            ];

            let block = Block::default()
                .title(Spans::from(vec![
                    Span::from("| "),
                    Span::from("Chose item to create:"),
                    Span::from(" |"),
                ]))
                .borders(Borders::ALL)
                .border_style(Style::default())
                .border_type(tui::widgets::BorderType::Thick)
                .style(Style::default().bg(tui::style::Color::Reset));

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default())
                .highlight_symbol(">>");

            frame.render_widget(Clear, layout);
            frame.render_stateful_widget(list, layout, &mut local_state.list_state);
        }
    }
}
