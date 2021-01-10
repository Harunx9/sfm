use crate::core::config::CoreConfig;

use self::{
    icon_cfg::IconsConfig, keyboard_cfg::KeyboardConfig,
    program_associations::FileAssociatedPrograms,
};

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
