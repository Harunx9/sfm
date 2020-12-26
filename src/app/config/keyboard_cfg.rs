use crossterm::event::{KeyCode, KeyModifiers};

use crate::core::key_binding::KeyBinding;

#[derive(Debug, Clone)]
pub struct KeyboardConfig {
    pub quit: KeyBinding,
    pub focus_left_panel: KeyBinding,
    pub focus_right_panel: KeyBinding,
    pub next_tab_item: KeyBinding,
    pub prev_tab_item: KeyBinding,
    pub next_tab: KeyBinding,
    pub prev_tab: KeyBinding,
    pub close_tab: KeyBinding,
    pub open: KeyBinding,
    pub open_as_tab: KeyBinding,
    pub navigate_up: KeyBinding,
    pub delete: KeyBinding,
    pub move_left: KeyBinding,
    pub move_right: KeyBinding,
    pub rename: KeyBinding,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            quit: KeyBinding::with_modifiers(KeyCode::Char('q'), KeyModifiers::CONTROL),
            focus_left_panel: KeyBinding::new(KeyCode::Char('h')),
            focus_right_panel: KeyBinding::new(KeyCode::Char('l')),
            next_tab_item: KeyBinding::new(KeyCode::Char('j')),
            prev_tab_item: KeyBinding::new(KeyCode::Char('k')),
            next_tab: KeyBinding::new(KeyCode::Char('n')),
            prev_tab: KeyBinding::new(KeyCode::Char('p')),
            close_tab: KeyBinding::new(KeyCode::Char('q')),
            open: KeyBinding::new(KeyCode::Char('o')),
            open_as_tab: KeyBinding::with_modifiers(KeyCode::Char('o'), KeyModifiers::CONTROL),
            navigate_up: KeyBinding::new(KeyCode::Backspace),
            delete: KeyBinding::with_modifiers(KeyCode::Char('d'), KeyModifiers::CONTROL),
            move_left: KeyBinding::with_modifiers(KeyCode::Char('h'), KeyModifiers::CONTROL),
            move_right: KeyBinding::with_modifiers(KeyCode::Char('l'), KeyModifiers::CONTROL),
            rename: KeyBinding::with_modifiers(KeyCode::Char('r'), KeyModifiers::CONTROL),
        }
    }
}
