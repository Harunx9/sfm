use std::fmt::Debug;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    app::{
        actions::{AppAction, FileManagerActions, PanelSide},
        file_system::FileSystem,
        state::{AppState, ModalType},
    },
    core::{
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
    },
};

use super::{
    create_modal::{CreateModalComponent, CreateModalProps},
    error_modal::{ErrorModalComponent, ErrorModalComponentProps},
    not_empty_dir_delete_modal::{
        NotEmptyDirDeleteModalComponent, NotEmptyDirDeleteModalComponentProps,
    },
    panel::PanelComponent,
    rename_modal::{RenameModalComponent, RenameModalComponentProps},
    ModalStyle,
};

#[derive(Clone, Default)]
pub struct RootComponentState {
    focused_panel: Option<PanelSide>,
}

pub struct RootComponent<TFileSystem: Clone + Debug + Default + FileSystem> {
    base: ComponentBase<(), RootComponentState>,
    left_panel: PanelComponent<TFileSystem>,
    right_panel: PanelComponent<TFileSystem>,
    create_modal: Option<CreateModalComponent<TFileSystem>>,
    rename_modal: Option<RenameModalComponent<TFileSystem>>,
    error_modal: Option<ErrorModalComponent<TFileSystem>>,
    non_empty_dir_delete_modal: Option<NotEmptyDirDeleteModalComponent<TFileSystem>>,
    _maker: std::marker::PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> RootComponent<TFileSystem> {
    pub fn new() -> Self {
        RootComponent {
            base: ComponentBase::new(None, Some(RootComponentState::default())),
            left_panel: PanelComponent::empty(),
            right_panel: PanelComponent::empty(),
            create_modal: None,
            rename_modal: None,
            error_modal: None,
            non_empty_dir_delete_modal: None,
            _maker: std::marker::PhantomData,
        }
    }

    fn map_state(&mut self, store: &Store<AppState<TFileSystem>, FileManagerActions>) {
        let state = store.get_state();
        if state.left_panel.is_focused {
            self.base.set_state(|_current_state| RootComponentState {
                focused_panel: Some(PanelSide::Left),
            });
        } else if state.right_panel.is_focused {
            self.base.set_state(|_current_state| RootComponentState {
                focused_panel: Some(PanelSide::Right),
            });
        } else {
            self.base.set_state(|_current_state| RootComponentState {
                focused_panel: None,
            });
        }
        self.left_panel = PanelComponent::with_panel_state(
            state.left_panel,
            PanelSide::Left,
            &state.config.icons,
            &state.config.core_cfg,
        );
        self.right_panel = PanelComponent::with_panel_state(
            state.right_panel,
            PanelSide::Right,
            &state.config.icons,
            &state.config.core_cfg,
        );
        if let Some(modal_type) = state.modal.clone() {
            match modal_type {
                ModalType::CreateModal {
                    item_index,
                    panel_side,
                    panel_tab,
                    panel_tab_path,
                } => {
                    if self.create_modal.is_none() {
                        self.create_modal =
                            Some(CreateModalComponent::with_props(CreateModalProps::new(
                                panel_side,
                                panel_tab,
                                panel_tab_path,
                                item_index,
                                state.config.icons.use_icons,
                                state.config.icons.get_file_icon("default".to_string()),
                                state.config.icons.get_dir_icon("default".to_string()),
                                state.config.icons.get_file_icon("symlink".to_string()),
                                state.config.core_cfg.list_arrow.clone(),
                                ModalStyle::new(
                                    state.config.core_cfg.color_scheme.normal_yellow,
                                    state.config.core_cfg.color_scheme.light_cyan,
                                    state.config.core_cfg.color_scheme.normal_black,
                                ),
                            )));
                    }
                }
                ModalType::ErrorModal(error_modal) => {
                    if self.error_modal.is_none() {
                        self.error_modal = Some(ErrorModalComponent::with_props(
                            ErrorModalComponentProps::new(
                                error_modal,
                                state.config.icons.use_icons,
                                state.config.icons.get_file_icon("warn".to_string()),
                            ),
                        ));
                    }
                }
                ModalType::RenameModal {
                    panel_side,
                    panel_tab,
                    item,
                } => {
                    if self.rename_modal.is_none() {
                        self.rename_modal = Some(RenameModalComponent::with_props(
                            RenameModalComponentProps::new(
                                Some(item),
                                Some(panel_side),
                                panel_tab,
                                ModalStyle::new(
                                    state.config.core_cfg.color_scheme.normal_yellow,
                                    state.config.core_cfg.color_scheme.light_cyan,
                                    state.config.core_cfg.color_scheme.normal_black,
                                ),
                            ),
                        ));
                    }
                }
                ModalType::DeleteDirWithContent {
                    panel_side,
                    panel_tab,
                    path,
                } => {
                    if self.non_empty_dir_delete_modal.is_none() {
                        self.non_empty_dir_delete_modal =
                            Some(NotEmptyDirDeleteModalComponent::new(
                                NotEmptyDirDeleteModalComponentProps::new(
                                    Some(panel_side),
                                    panel_tab,
                                    path,
                                    state.config.core_cfg.list_arrow.clone(),
                                    ModalStyle::new(
                                        state.config.core_cfg.color_scheme.normal_yellow,
                                        state.config.core_cfg.color_scheme.light_cyan,
                                        state.config.core_cfg.color_scheme.normal_black,
                                    ),
                                ),
                            ));
                    }
                }
            };
        }
        if self.create_modal.is_some() && state.modal.is_none() {
            self.create_modal = None;
        }

        if self.rename_modal.is_some() && state.modal.is_none() {
            self.rename_modal = None;
        }

        if self.error_modal.is_some() && state.modal.is_none() {
            self.error_modal = None;
        }

        if self.non_empty_dir_delete_modal.is_some() && state.modal.is_none() {
            self.non_empty_dir_delete_modal = None;
        }
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    Component<Event, AppState<TFileSystem>, FileManagerActions> for RootComponent<TFileSystem>
{
    fn on_tick(&mut self, store: &mut Store<AppState<TFileSystem>, FileManagerActions>) {
        self.left_panel.on_tick(store);
        self.right_panel.on_tick(store);

        if store.is_dirty() {
            self.map_state(store);
            store.clean();
        }
    }

    fn on_init(&mut self, store: &Store<AppState<TFileSystem>, FileManagerActions>) {
        self.map_state(store);
    }

    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();

        if self.left_panel.tab_in_search_mode() == false
            && self.right_panel.tab_in_search_mode() == false
        {
            if let Event::Keyboard(key_evt) = event {
                if state.config.keyboard_cfg.quit.is_pressed(key_evt) {
                    store.dispatch(FileManagerActions::App(AppAction::Exit));
                    return true;
                }

                if let Some(ref mut error_modal) = self.error_modal {
                    let result = error_modal.handle_event(event, store);
                    self.map_state(store);
                    store.clean();

                    return result;
                }

                if let Some(ref mut non_empty_dir_delete_modal) = self.non_empty_dir_delete_modal {
                    let result = non_empty_dir_delete_modal.handle_event(event, store);
                    self.map_state(store);
                    store.clean();

                    return result;
                }

                if let Some(ref mut create_modal) = self.create_modal {
                    let result = create_modal.handle_event(event, store);
                    self.map_state(store);

                    return result;
                }

                if let Some(ref mut rename_modal) = self.rename_modal {
                    let result = rename_modal.handle_event(event, store);
                    self.map_state(store);
                    store.clean();

                    return result;
                }

                if state
                    .config
                    .keyboard_cfg
                    .focus_left_panel
                    .is_pressed(key_evt)
                {
                    store.dispatch(FileManagerActions::App(AppAction::FocusLeft));
                    self.map_state(store);
                    store.clean();

                    return true;
                }

                if state
                    .config
                    .keyboard_cfg
                    .focus_right_panel
                    .is_pressed(key_evt)
                {
                    store.dispatch(FileManagerActions::App(AppAction::FocusRight));
                    self.map_state(store);
                    store.clean();

                    return true;
                }
            }
        }

        let mut result = self.left_panel.handle_event(event, store);
        if result == true {
            self.map_state(store);
            store.clean();

            return result;
        }
        result = self.right_panel.handle_event(event, store);
        self.map_state(store);
        store.clean();

        result
    }

    fn render<TBackend: Backend>(&self, frame: &mut tui::Frame<TBackend>, _area: Option<Rect>) {
        let local_state = self.base.get_state().unwrap();
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());
        self.left_panel.render(frame, Some(layout[0]));
        self.right_panel.render(frame, Some(layout[1]));
        if let Some(ref create_modal) = self.create_modal {
            if let Some(focused_panel) = local_state.focused_panel.clone() {
                match focused_panel {
                    PanelSide::Left => create_modal.render(frame, Some(layout[0])),
                    PanelSide::Right => create_modal.render(frame, Some(layout[1])),
                };
            } else {
                create_modal.render(frame, None);
            }
        }

        if let Some(ref rename_modal) = self.rename_modal {
            if let Some(focused_panel) = local_state.focused_panel.clone() {
                match focused_panel {
                    PanelSide::Left => rename_modal.render(frame, Some(layout[0])),
                    PanelSide::Right => rename_modal.render(frame, Some(layout[1])),
                };
            } else {
                rename_modal.render(frame, None);
            }
        }

        if let Some(ref non_empty_dir_delete_modal) = self.non_empty_dir_delete_modal {
            if let Some(focused_panel) = local_state.focused_panel.clone() {
                match focused_panel {
                    PanelSide::Left => non_empty_dir_delete_modal.render(frame, Some(layout[0])),
                    PanelSide::Right => non_empty_dir_delete_modal.render(frame, Some(layout[1])),
                };
            } else {
                non_empty_dir_delete_modal.render(frame, None);
            }
        }

        if let Some(ref error_modal) = self.error_modal {
            if let Some(focused_panel) = local_state.focused_panel.clone() {
                match focused_panel {
                    PanelSide::Left => error_modal.render(frame, Some(layout[0])),
                    PanelSide::Right => error_modal.render(frame, Some(layout[1])),
                };
            } else {
                error_modal.render(frame, None);
            }
        }

        if let Some(ref error_modal) = self.error_modal {
            if let Some(focused_panel) = local_state.focused_panel.clone() {
                match focused_panel {
                    PanelSide::Left => error_modal.render(frame, Some(layout[0])),
                    PanelSide::Right => error_modal.render(frame, Some(layout[1])),
                };
            } else {
                error_modal.render(frame, None);
            }
        }
    }
}
