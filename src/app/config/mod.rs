use toml::Value;

use crate::core::config::CoreConfig;
use std::fs;

use self::{
    icon_cfg::IconsConfig, keyboard_cfg::KeyboardConfig,
    program_associations::FileAssociatedPrograms,
};

use super::file_system::path::expand_if_contains_tilde;

pub mod icon_cfg;
pub mod keyboard_cfg;
pub mod program_associations;

#[derive(Debug, Clone)]
pub struct Config {
    pub core_cfg: CoreConfig,
    pub keyboard_cfg: KeyboardConfig,
    pub icons: IconsConfig,
    pub file_associated_programs: FileAssociatedPrograms,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            core_cfg: CoreConfig::default(),
            keyboard_cfg: KeyboardConfig::default(),
            icons: IconsConfig::default(),
            file_associated_programs: FileAssociatedPrograms::default(),
        }
    }
}

impl Config {
    pub fn load_or_default(paths: Vec<String>) -> Self {
        let mut cfg = Config::default();
        if let Some(config_content) = read_config_file_to_string(paths) {
            if let Ok(toml_mapped_values) = config_content.parse::<Value>() {
                cfg.icons.update_from_file(&toml_mapped_values);
                cfg.keyboard_cfg.update_from_file(&toml_mapped_values);
                cfg.file_associated_programs
                    .update_from_file(&toml_mapped_values);
                cfg.core_cfg.update_from_file(&toml_mapped_values);
                println!("cfg updated");
                println!("{:?}", cfg);
            }
        }
        cfg
    }
}

fn read_config_file_to_string(paths: Vec<String>) -> Option<String> {
    for path in paths {
        if let Some(path) = expand_if_contains_tilde(path) {
            match fs::read_to_string(path) {
                Ok(content) => return Some(content.clone()),
                Err(_) => continue,
            }
        }
    }
    None
}
