use std::{
    error::Error,
    io::{stdout, Write},
};

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{config::Config, events::EventQueue, state::State};

pub struct App {
    state: State,
    config: Config,
}

impl App {
    pub fn new() -> Self {
        App {
            state: State::default(),
            config: Config::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);

        let mut terminal = Terminal::new(backend);
        let event_queue = EventQueue::start_with_config(self.config);

        loop {
            if let Ok(event) = event_queue.pool() {}
            if self.state.app_exit {
                break;
            }
        }

        Ok(())
    }
}
