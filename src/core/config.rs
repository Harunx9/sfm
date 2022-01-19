use toml::Value;

use super::color_scheme::ColorScheme;

#[derive(Clone, Debug)]
pub struct CoreConfig {
    pub tick_rate: u64,
    pub color_scheme: ColorScheme,
    pub list_arrow: String,
}

impl Default for CoreConfig {
    fn default() -> Self {
        CoreConfig {
            tick_rate: 240,
            color_scheme: ColorScheme::default(),
            list_arrow: "".to_string(),
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

                if let Some(list_arrow) = core.get("list_arrow") {
                    if let Value::String(list_arrow) = list_arrow {
                        self.list_arrow = list_arrow.clone();
                    }
                }
            }
        }

        if let Some(color_scheme) = cfg.get("color_scheme") {
            self.color_scheme.update_from_file(color_scheme);
        }
    }
}
