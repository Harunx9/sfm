use tui::{backend::Backend, layout::Rect, Frame};

use crate::core::store::Store;

pub trait Component<TEvent, TGlobalState, TAction>
where
    TEvent: Clone,
    TGlobalState: Default + Clone,
    TAction: Clone,
{
    fn on_init(&mut self, store: &Store<TGlobalState, TAction>) {}
    fn handle_event(&mut self, event: TEvent, store: &mut Store<TGlobalState, TAction>) -> bool;
    fn on_tick(&mut self) {}
    fn render<TBackend: Backend>(&self, frame: &mut Frame<TBackend>, area: Option<Rect>);
}
