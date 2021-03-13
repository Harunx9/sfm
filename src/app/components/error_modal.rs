use crate::{
    app::{
        actions::{AppAction, FileManagerActions},
        file_system::FileSystem,
        state::AppState,
    },
    core::{
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
    },
};
use std::fmt::Debug;
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use super::create_modal_layout;

#[derive(Clone, Default)]
pub struct ErrorModalComponentProps {
    message: Option<String>,
    show_icons: bool,
    error_icon: String,
}

impl ErrorModalComponentProps {
    pub fn new(message: String, show_icons: bool, error_icon: String) -> Self {
        ErrorModalComponentProps {
            message: Some(message),
            show_icons,
            error_icon,
        }
    }
}

pub struct ErrorModalComponent<TFileSystem: Clone + Debug + Default + FileSystem> {
    base: ComponentBase<ErrorModalComponentProps, ()>,
    _maker: std::marker::PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> ErrorModalComponent<TFileSystem> {
    pub fn with_props(props: ErrorModalComponentProps) -> Self {
        ErrorModalComponent {
            base: ComponentBase::new(Some(props), None),
            _maker: std::marker::PhantomData,
        }
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    Component<Event, AppState<TFileSystem>, FileManagerActions>
    for ErrorModalComponent<TFileSystem>
{
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        if let Event::Keyboard(key_evt) = event {
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
        let props = self.base.get_props().unwrap();
        let message = if let Some(message) = props.message {
            message.clone()
        } else {
            "".to_string()
        };
        let block = Block::default()
            .title(Spans::from(vec![
                Span::from("| "),
                Span::from("Error: (Esc to close)"),
                Span::from(" |"),
            ]))
            .borders(Borders::ALL)
            .border_style(Style::default())
            .border_type(BorderType::Thick)
            .style(Style::default());

        let paragraph = Paragraph::new(message)
            .block(block)
            .alignment(tui::layout::Alignment::Center);

        frame.render_widget(Clear, layout);
        frame.render_widget(paragraph, layout);
    }
}
