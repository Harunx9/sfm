use toml::Value;

use crate::core::config::CoreConfig;
use std::path::Path;

use self::{
    icon_cfg::IconsConfig, keyboard_cfg::KeyboardConfig,
    program_associations::FileAssociatedPrograms,
};

use super::file_system::{functions::expand_if_contains_tilde, FileSystem};

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
    pub fn load_or_default<TPath: AsRef<Path>, TFileSystem: FileSystem>(
        paths: Vec<TPath>,
        file_system: &TFileSystem,
    ) -> Self {
        let mut cfg = Config::default();
        if let Some(config_content) = read_config_file_to_string(paths, file_system) {
            if let Ok(toml_mapped_values) = config_content.parse::<Value>() {
                cfg.icons.update_from_file(&toml_mapped_values);
                cfg.keyboard_cfg.update_from_file(&toml_mapped_values);
                cfg.file_associated_programs
                    .update_from_file(&toml_mapped_values);
                cfg.core_cfg.update_from_file(&toml_mapped_values);
            }
        }
        cfg
    }
}

fn read_config_file_to_string<TPath: AsRef<Path>, TFileSystem: FileSystem>(
    paths: Vec<TPath>,
    file_system: &TFileSystem,
) -> Option<String> {
    for path in paths {
        if let Some(path) = expand_if_contains_tilde(path) {
            match file_system.read_to_string(&path) {
                Some(content) => return Some(content.clone()),
                None => continue,
            }
        }
    }
    None
}
