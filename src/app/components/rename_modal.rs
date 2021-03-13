use std::{fmt::Debug, path::PathBuf};

use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{
    app::{
        actions::{
            AppAction, DirectoryAction, FileAction, FileManagerActions, PanelInfo, PanelSide,
        },
        file_system::{file_system_item::FileSystemItem, FileSystem},
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
pub struct RenameModalComponentProps {
    item_to_rename: Option<FileSystemItem>,
    panel_side: Option<PanelSide>,
    panel_tab: TabIdx,
    modal_style: ModalStyle,
}

impl RenameModalComponentProps {
    pub fn new(
        item_to_rename: Option<FileSystemItem>,
        panel_side: Option<PanelSide>,
        panel_tab: TabIdx,
        modal_style: ModalStyle,
    ) -> Self {
        Self {
            item_to_rename,
            panel_side,
            panel_tab,
            modal_style,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RenameModalComponentState {
    input: String,
}

pub struct RenameModalComponent<TFileSystem: Clone + Debug + Default + FileSystem> {
    base: ComponentBase<RenameModalComponentProps, RenameModalComponentState>,
    _maker: std::marker::PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> RenameModalComponent<TFileSystem> {
    pub fn with_props(props: RenameModalComponentProps) -> Self {
        let item = props.item_to_rename.clone().unwrap();

        RenameModalComponent {
            base: ComponentBase::new(
                Some(props),
                Some(RenameModalComponentState {
                    input: item.get_path().to_str().unwrap().to_string(),
                }),
            ),
            _maker: std::marker::PhantomData,
        }
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    Component<Event, AppState<TFileSystem>, FileManagerActions>
    for RenameModalComponent<TFileSystem>
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
            if state.config.keyboard_cfg.accept.is_pressed(key_evt)
                && local_state.input.is_empty() == false
            {
                let panel_side = props.panel_side.unwrap();
                let item = props.item_to_rename.unwrap();
                match item {
                    FileSystemItem::Directory(dir) => {
                        store.dispatch(FileManagerActions::Directory(DirectoryAction::Rename {
                            from: PanelInfo {
                                side: panel_side.clone(),
                                tab: props.panel_tab,
                                path: dir.get_path(),
                            },
                            to: PanelInfo {
                                side: panel_side.clone(),
                                tab: props.panel_tab,
                                path: PathBuf::from(local_state.input),
                            },
                        }));
                    }
                    FileSystemItem::File(file) => {
                        store.dispatch(FileManagerActions::File(FileAction::Rename {
                            from: PanelInfo {
                                side: panel_side.clone(),
                                tab: props.panel_tab,
                                path: file.get_path(),
                            },
                            to: PanelInfo {
                                side: panel_side.clone(),
                                tab: props.panel_tab,
                                path: PathBuf::from(local_state.input),
                            },
                        }));
                    }
                    FileSystemItem::Symlink(_) => {}
                    FileSystemItem::Unknown => {}
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

                        RenameModalComponentState {
                            input: current_text,
                        }
                    });
                    return true;
                }
                KeyCode::Backspace => {
                    self.base.set_state(|current_state| {
                        let mut current_text = current_state.input.clone();
                        current_text.pop();

                        RenameModalComponentState {
                            input: current_text,
                        }
                    });
                    return true;
                }
                _ => {}
            };

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

        let local_state = self.base.get_state().unwrap();
        let block = Block::default()
            .title(Spans::from(vec![
                Span::from("| "),
                Span::from("Item name:"),
                Span::from(" |"),
            ]))
            .borders(Borders::ALL)
            .border_style(Style::default())
            .border_type(tui::widgets::BorderType::Thick)
            .style(Style::default());

        let paragraph = Paragraph::new(local_state.input)
            .block(block)
            .alignment(tui::layout::Alignment::Center);

        frame.render_widget(Clear, layout);
        frame.render_widget(paragraph, layout);
    }
}
