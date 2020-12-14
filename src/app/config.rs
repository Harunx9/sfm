use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::core::config::CoreConfig;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub core_cfg: CoreConfig,
    pub enchanced_graphics: bool,
    pub keyboard_cfg: KeyboardConfig,
    pub icons: Icons,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            core_cfg: CoreConfig::default(),
            enchanced_graphics: false,
            keyboard_cfg: KeyboardConfig::default(),
            icons: Icons::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardConfig {
    pub quit: KeyBinging,
    pub focus_left_panel: KeyBinging,
    pub focus_right_panel: KeyBinging,
    pub next_tab_item: KeyBinging,
    pub prev_tab_item: KeyBinging,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            quit: KeyBinging::new(KeyCode::Char('q'), None),
            focus_left_panel: KeyBinging::new(KeyCode::Char('h'), None),
            focus_right_panel: KeyBinging::new(KeyCode::Char('l'), None),
            next_tab_item: KeyBinging::new(KeyCode::Char('j'), None),
            prev_tab_item: KeyBinging::new(KeyCode::Char('k'), None),
        }
    }
}

pub struct Icons {
    dir_icons: HashMap<String, String>,
    files_icon: HashMap<String, String>,
}

impl Default for Icons {
    fn default() -> Self {
        Icons {
            dir_icons: get_default_dir_icons(),
            files_icon: get_default_files_icons(),
        }
    }
}

fn get_default_dir_icons() -> HashMap<String, String> {
    let icon_map = HashMap::new();
    icon_map.insert(".git", "");
    icon_map.insert("node_modules", "");
    icon_map.insert("default", "");

    icon_map
}

fn get_default_files_icons() -> HashMap<String, String> {
    let icon_map = HashMap::new();
    //GIT
    icon_map.insert(".gitignore", "");
    icon_map.insert(".gitmodules", "");

    //PROGRAMMING LANGUAGES
    icon_map.insert("rs", "");
    icon_map.insert("cs", "");
    icon_map.insert("cpp", "ﭱ");
    icon_map.insert("hpp", "");
    icon_map.insert("h", "");
    icon_map.insert("js", "");
    icon_map.insert("ts", "");
    icon_map.insert("jsx", "");
    icon_map.insert("tsx", "ﰆ");
    icon_map.insert("html", "");
    icon_map.insert("css", "");
    icon_map.insert("sass", "");
    icon_map.insert("toml", "");
    icon_map.insert("yaml", "");
    icon_map.insert("php", "");
    icon_map.insert("py", "");
    icon_map.insert("rb", "");
    icon_map.insert("java", "");
    icon_map.insert("default", "");

    icon_map
}

impl Icons {
    pub fn get_dir_icon(&self, dir_name: String) -> Option<&str> {
        &self.dir_icons.get(dir_name)
    }

    pub fn get_file_icon(&self, file_name: String) -> Option<&str> {
        &self.files_icon.get(file_name)
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
