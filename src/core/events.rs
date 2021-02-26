use crossterm::event::{self, KeyEvent, MouseEvent};
use std::{
    sync::mpsc::RecvError,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver},
        Arc,
    },
    thread,
    thread::JoinHandle,
    time::Duration,
    time::Instant,
};

use super::config::CoreConfig;
#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyEvent),
    Resize(Size),
    Error(Error),
    Tick,
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    MessagePoolError,
    EventReadError,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::MessagePoolError => "Error happend on message pool".to_string(),
            Error::EventReadError => "Error on event read".to_string(),
        }
    }
}

pub struct EventQueue {
    receiver: Receiver<Event>,
    skip_input_event: Arc<AtomicBool>,
    _runner_handle: JoinHandle<()>,
}

impl EventQueue {
    pub fn start() -> Self {
        EventQueue::start_with_config(CoreConfig::default())
    }

    pub fn start_with_config(config: CoreConfig) -> Self {
        let (sender, receiver) = channel();
        let tick_rate = Duration::from_millis(config.tick_rate);
        let skip_input_event = Arc::new(AtomicBool::new(false));

        let skip_event_read = skip_input_event.clone();
        let runner_handle = thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_millis(0));

                if skip_event_read.load(Ordering::Relaxed) {
                    continue;
                }

                if event::poll(timeout).unwrap() {
                    match event::read() {
                        Ok(event) => {
                            match event {
                                event::Event::Key(key) => {
                                    sender.send(Event::Keyboard(key)).unwrap();
                                }
                                event::Event::Mouse(mouse) => {
                                    sender.send(Event::Mouse(mouse)).unwrap();
                                }
                                event::Event::Resize(width, height) => {
                                    sender.send(Event::Resize(Size { width, height })).unwrap();
                                }
                            };
                        }
                        Err(_err) => {
                            sender.send(Event::Error(Error::EventReadError)).unwrap();
                        }
                    };
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        EventQueue {
            receiver,
            skip_input_event: skip_input_event.clone(),
            _runner_handle: runner_handle,
        }
    }

    pub fn lock_event_read(&mut self) {
        self.skip_input_event.store(true, Ordering::Relaxed);
    }

    pub fn unlock_event_read(&mut self) {
        self.skip_input_event.store(false, Ordering::Relaxed);
    }

    pub fn pool(&self) -> Result<Event, RecvError> {
        self.receiver.recv()
    }
}
