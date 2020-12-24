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
