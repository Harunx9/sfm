use tui::{backend::Backend, layout::Rect, Frame};

use crate::core::store::Store;

pub trait Component<TEvent, TGlobalState, TAction>
where
    TEvent: Clone,
    TGlobalState: Default + Clone,
    TAction: Clone,
{
    fn on_init(&mut self, _store: &Store<TGlobalState, TAction>) {}
    fn handle_event(&mut self, _event: TEvent, _store: &mut Store<TGlobalState, TAction>) -> bool {
        true
    }
    fn on_tick(&mut self, _store: &mut Store<TGlobalState, TAction>) {}
    fn render<TBackend: Backend>(&self, frame: &mut Frame<TBackend>, area: Option<Rect>);
}
