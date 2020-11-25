use crossterm::event::{self, KeyEvent, MouseEvent};
use std::{
    sync::mpsc::RecvError,
    sync::mpsc::{channel, Receiver},
    thread,
    thread::JoinHandle,
    time::Duration,
    time::Instant,
};

use crate::config::Config;

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
    runner_handle: JoinHandle<()>,
}

impl EventQueue {
    pub fn start() -> Self {
        EventQueue::start_with_config(Config::default())
    }

    pub fn start_with_config(config: Config) -> Self {
        let (sender, receiver) = channel();
        let tick_rate = Duration::from_millis(config.tick_rate);
        let runner_handle = thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_millis(0));

                match event::poll(timeout) {
                    Ok(_pool) => match event::read() {
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
                    },
                    Err(_err) => {
                        sender.send(Event::Error(Error::MessagePoolError)).unwrap();
                    }
                };

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        EventQueue {
            receiver,
            runner_handle,
        }
    }

    pub fn pool(&self) -> Result<Event, RecvError> {
        self.receiver.recv()
    }
}
