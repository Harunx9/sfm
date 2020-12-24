use tui::style::Color;

#[derive(Clone, Copy, Debug)]
pub struct ColorScheme {
    pub foregorund: Color,
    pub background: Color,

    pub normal_black: Color,
    pub normal_red: Color,
    pub normal_green: Color,
    pub normal_yellow: Color,
    pub normal_blue: Color,
    pub normal_magneta: Color,
    pub normal_cyan: Color,
    pub normal_white: Color,

    pub light_black: Color,
    pub light_red: Color,
    pub light_green: Color,
    pub light_yellow: Color,
    pub light_blue: Color,
    pub light_magneta: Color,
    pub light_cyan: Color,
    pub light_white: Color,
}

impl ColorScheme {
    pub fn new(
        foregorund: Color,
        background: Color,
        normal_black: Color,
        normal_red: Color,
        normal_green: Color,
        normal_yellow: Color,
        normal_blue: Color,
        normal_magneta: Color,
        normal_cyan: Color,
        normal_white: Color,
        light_black: Color,
        light_red: Color,
        light_green: Color,
        light_yellow: Color,
        light_blue: Color,
        light_magneta: Color,
        light_cyan: Color,
        light_white: Color,
    ) -> Self {
        Self {
            foregorund,
            background,
            normal_black,
            normal_red,
            normal_green,
            normal_yellow,
            normal_blue,
            normal_magneta,
            normal_cyan,
            normal_white,
            light_black,
            light_red,
            light_green,
            light_yellow,
            light_blue,
            light_magneta,
            light_cyan,
            light_white,
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme::new(
            Color::White,
            Color::Reset,
            Color::Black,
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
            Color::Black,
            Color::LightRed,
            Color::LightGreen,
            Color::LightYellow,
            Color::LightBlue,
            Color::LightMagenta,
            Color::LightCyan,
            Color::White,
        )
    }
}
