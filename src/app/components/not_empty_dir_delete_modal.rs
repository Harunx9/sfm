use tui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, ListState},
    Frame,
};

use crate::{
    app::{
        actions::{AppAction, PanelSide},
        state::TabIdx,
    },
    core::{store::Store, ui::component_base::ComponentBase},
};
use crate::{
    app::{
        actions::{DirectoryAction, FileManagerActions, PanelInfo},
        file_system::FileSystem,
        state::AppState,
    },
    core::{events::Event, ui::component::Component},
};
use std::{fmt::Debug, marker::PhantomData, path::PathBuf};

use super::{create_modal_layout, ModalStyle};

#[derive(Clone, Default)]
pub struct NotEmptyDirDeleteModalComponentProps {
    panel_side: Option<PanelSide>,
    panel_tab: TabIdx,
    path: PathBuf,
    list_selector: String,
    modal_style: ModalStyle,
}

impl NotEmptyDirDeleteModalComponentProps {
    pub fn new(
        panel_side: Option<PanelSide>,
        panel_tab: TabIdx,
        path: PathBuf,
        list_selector: String,
        modal_style: ModalStyle,
    ) -> Self {
        NotEmptyDirDeleteModalComponentProps {
            panel_side,
            panel_tab,
            path,
            list_selector,
            modal_style,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Options {
    Ok,
    Cancel,
}

impl ToString for Options {
    fn to_string(&self) -> String {
        match self {
            Options::Ok => "Ok".to_string(),
            Options::Cancel => "Cancel".to_string(),
        }
    }
}

impl From<String> for Options {
    fn from(source: String) -> Self {
        match source.as_str() {
            "Ok" => Options::Ok,
            "Cancel" => Options::Cancel,
            _ => panic!(""),
        }
    }
}

impl From<usize> for Options {
    fn from(source: usize) -> Self {
        match source {
            0 => Options::Ok,
            1 => Options::Cancel,
            _ => panic!(""),
        }
    }
}

#[derive(Clone, Default)]
pub struct NotEmptyDirDeleteModalComponentState {
    list_state: ListState,
}

pub struct NotEmptyDirDeleteModalComponent<TFileSystem: Clone + Debug + Default + FileSystem> {
    base: ComponentBase<NotEmptyDirDeleteModalComponentProps, NotEmptyDirDeleteModalComponentState>,
    _marker: PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    NotEmptyDirDeleteModalComponent<TFileSystem>
{
    pub fn new(props: NotEmptyDirDeleteModalComponentProps) -> Self {
        NotEmptyDirDeleteModalComponent {
            base: ComponentBase::new(
                Some(props),
                Some(NotEmptyDirDeleteModalComponentState::default()),
            ),
            _marker: PhantomData,
        }
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    Component<Event, AppState<TFileSystem>, FileManagerActions>
    for NotEmptyDirDeleteModalComponent<TFileSystem>
{
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        let local_state = self.base.get_state().unwrap();
        if let Event::Keyboard(key_evt) = event {
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
                    NotEmptyDirDeleteModalComponentState {
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
                    NotEmptyDirDeleteModalComponentState {
                        list_state: current_state.list_state,
                        ..current_state
                    }
                });
                return true;
            }

            if state.config.keyboard_cfg.accept.is_pressed(key_evt) {
                if let Some(selected) = local_state.list_state.selected() {
                    let props = self.base.get_props().unwrap();
                    let option = Options::from(selected);
                    match option {
                        Options::Ok => store.dispatch(FileManagerActions::Directory(
                            DirectoryAction::DeleteWithContent {
                                panel: PanelInfo {
                                    side: props.panel_side.unwrap(),
                                    tab: props.panel_tab,
                                    path: props.path,
                                },
                            },
                        )),
                        _ => {}
                    }
                    store.dispatch(FileManagerActions::App(AppAction::CloseModal));
                }
            }

            if state.config.keyboard_cfg.close.is_pressed(key_evt) {
                store.dispatch(FileManagerActions::App(AppAction::CloseModal));
                return true;
            }
        }
        false
    }

    fn render<TBackend: Backend>(&self, frame: &mut Frame<TBackend>, area: Option<Rect>) {
        let layout = if let Some(area) = area {
            create_modal_layout(50, 10, area)
        } else {
            create_modal_layout(50, 10, frame.size())
        };

        let props = self.base.get_props().unwrap();
        let mut local_state = self.base.get_state().unwrap();

        let items = vec![
            ListItem::new(Spans::from(vec![Span::from(Options::Ok.to_string())])),
            ListItem::new(Spans::from(vec![Span::from(Options::Cancel.to_string())])),
        ];

        let block = Block::default()
            .title(Spans::from(vec![
                Span::from("| "),
                Span::from("This directory is not empty do you want to remove it ?"),
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
                    .bg(props.modal_style.selected_element_background)
                    .fg(props.modal_style.selected_element_foreground),
            )
            .highlight_symbol(props.list_selector.as_str());

        frame.render_widget(Clear, layout);
        frame.render_stateful_widget(list, layout, &mut local_state.list_state);
    }
}
