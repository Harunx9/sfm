#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: u64,
    pub enchanced_graphics: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tick_rate: 240,
            enchanced_graphics: false,
        }
    }
}
