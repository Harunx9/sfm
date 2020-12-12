use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::core::config::CoreConfig;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub core_cfg: CoreConfig,
    pub enchanced_graphics: bool,
    pub keyboard_cfg: KeyboardConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            core_cfg: CoreConfig::default(),
            enchanced_graphics: false,
            keyboard_cfg: KeyboardConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardConfig {
    pub quit: KeyBinging,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            quit: KeyBinging::new(KeyCode::Char('q'), None),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyBinging {
    key: KeyCode,
    modifier: Option<KeyModifiers>,
}

impl KeyBinging {
    pub fn new(key: KeyCode, modifier: Option<KeyModifiers>) -> Self {
        KeyBinging { key, modifier }
    }

    pub fn is_pressed(&self, key_evt: KeyEvent) -> bool {
        match self.modifier {
            Some(modifier) => modifier == key_evt.modifiers && self.key == key_evt.code,
            None => self.key == key_evt.code,
        }
    }
}
