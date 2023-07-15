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
        match key_evt.code {
            KeyCode::Char(ch) => {
                self.modifiers == key_evt.modifiers
                    && self.key
                        == KeyCode::Char(ch.to_lowercase().to_string().chars().next().unwrap())
            }
            _ => self.modifiers == key_evt.modifiers && self.key == key_evt.code,
        }
    }
}
