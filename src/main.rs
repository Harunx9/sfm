use crate::core::{events::EventQueue, store::Store};
use std::{
    error::Error,
    io::{stdout, Write},
};

use app::{actions::FrActions, config::Config, reducers::root_reducer, state::State};
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::default();
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    let event_queue = EventQueue::start_with_config(cfg.core_cfg);
    let store = Store::<State, FrActions>::new(root_reducer);

    terminal.clear()?;

    loop {
        if let Ok(event) = event_queue.pool() {}
        let state = store.get_state();
        if state.app_exit {
            break;
        }
    }

    Ok(())
}
