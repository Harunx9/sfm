use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
