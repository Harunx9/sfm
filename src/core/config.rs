use toml::Value;

use super::color_scheme::ColorScheme;

#[derive(Clone, Copy, Debug)]
pub struct CoreConfig {
    pub tick_rate: u64,
    pub color_scheme: ColorScheme,
}

impl Default for CoreConfig {
    fn default() -> Self {
        CoreConfig {
            tick_rate: 240,
            color_scheme: ColorScheme::default(),
        }
    }
}

impl CoreConfig {
    pub fn update_from_file(&mut self, cfg: &Value) {
        if let Some(core) = cfg.get("core") {
            if let Value::Table(core) = core {
                if let Some(tick_rate) = core.get("tick_rate") {
                    if let Value::Integer(tick_rate) = tick_rate {
                        self.tick_rate = tick_rate.clone() as u64;
                    }
                }
            }
        }

        if let Some(color_scheme) = cfg.get("color_scheme") {
            self.color_scheme.update_from_file(color_scheme);
        }
    }
}
