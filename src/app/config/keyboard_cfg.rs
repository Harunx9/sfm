use crossterm::event::{KeyCode, KeyModifiers};

use crate::core::key_binding::KeyBinging;

#[derive(Debug, Clone)]
pub struct KeyboardConfig {
    pub quit: KeyBinging,
    pub focus_left_panel: KeyBinging,
    pub focus_right_panel: KeyBinging,
    pub next_tab_item: KeyBinging,
    pub prev_tab_item: KeyBinging,
    pub next_tab: KeyBinging,
    pub prev_tab: KeyBinging,
    pub close_tab: KeyBinging,
    pub open: KeyBinging,
    pub open_as_tab: KeyBinging,
    pub navigate_up: KeyBinging,
    pub delete: KeyBinging,
    pub move_left: KeyBinging,
    pub move_right: KeyBinging,
    pub rename: KeyBinging,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            quit: KeyBinging::new(KeyCode::Char('q'), Some(KeyModifiers::CONTROL)),
            focus_left_panel: KeyBinging::new(KeyCode::Char('h'), None),
            focus_right_panel: KeyBinging::new(KeyCode::Char('l'), None),
            next_tab_item: KeyBinging::new(KeyCode::Char('j'), None),
            prev_tab_item: KeyBinging::new(KeyCode::Char('k'), None),
            next_tab: KeyBinging::new(KeyCode::Char('n'), None),
            prev_tab: KeyBinging::new(KeyCode::Char('p'), None),
            close_tab: KeyBinging::new(KeyCode::Char('q'), None),
            open: KeyBinging::new(KeyCode::Char('o'), None),
            open_as_tab: KeyBinging::new(KeyCode::Char('o'), Some(KeyModifiers::CONTROL)),
            navigate_up: KeyBinging::new(KeyCode::Backspace, None),
            delete: KeyBinging::new(KeyCode::Char('d'), Some(KeyModifiers::CONTROL)),
            move_left: KeyBinging::new(KeyCode::Char('l'), Some(KeyModifiers::CONTROL)),
            move_right: KeyBinging::new(KeyCode::Char('l'), Some(KeyModifiers::CONTROL)),
            rename: KeyBinging::new(KeyCode::Char('r'), Some(KeyModifiers::CONTROL)),
        }
    }
}
