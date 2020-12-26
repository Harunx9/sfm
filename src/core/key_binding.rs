use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy)]
pub struct KeyBinding {
    key: KeyCode,
    modifiers: KeyModifiers,
}

impl KeyBinding {
    pub fn with_modifiers(key: KeyCode, modifiers: KeyModifiers) -> Self {
        KeyBinding { key, modifiers }
    }

    pub fn new(key: KeyCode) -> Self {
        KeyBinding {
            key,
            modifiers: KeyModifiers::empty(),
        }
    }

    pub fn is_pressed(&self, key_evt: KeyEvent) -> bool {
        self.modifiers == key_evt.modifiers && self.key == key_evt.code
    }
}
