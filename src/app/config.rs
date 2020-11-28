use crate::core::config::CoreConfig;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub core_cfg: CoreConfig,
    pub enchanced_graphics: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            core_cfg: CoreConfig::default(),
            enchanced_graphics: false,
        }
    }
}
