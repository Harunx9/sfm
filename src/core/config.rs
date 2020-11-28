#[derive(Clone, Copy, Debug)]
pub struct CoreConfig {
    pub tick_rate: u64,
}

impl Default for CoreConfig {
    fn default() -> Self {
        CoreConfig { tick_rate: 240 }
    }
}
