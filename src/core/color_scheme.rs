use toml::Value;
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

    pub fn update_from_file(&mut self, cfg: &Value) {
        if let Some(foregorund) = cfg.get("foregorund") {
            self.foregorund = map_color(&foregorund);
        }

        if let Some(background) = cfg.get("background") {
            self.background = map_color(&background);
        }

        if let Some(normal_black) = cfg.get("normal_black") {
            self.normal_black = map_color(&normal_black);
        }

        if let Some(normal_red) = cfg.get("normal_red") {
            self.normal_red = map_color(&normal_red);
        }

        if let Some(normal_green) = cfg.get("normal_green") {
            self.normal_green = map_color(&normal_green);
        }

        if let Some(normal_yellow) = cfg.get("normal_yellow") {
            self.normal_yellow = map_color(&normal_yellow);
        }

        if let Some(normal_blue) = cfg.get("normal_blue") {
            self.normal_blue = map_color(&normal_blue);
        }

        if let Some(normal_magneta) = cfg.get("normal_magneta") {
            self.normal_magneta = map_color(&normal_magneta);
        }

        if let Some(normal_cyan) = cfg.get("normal_cyan") {
            self.normal_cyan = map_color(&normal_cyan);
        }

        if let Some(normal_white) = cfg.get("normal_white") {
            self.normal_white = map_color(&normal_white);
        }

        if let Some(light_black) = cfg.get("light_black") {
            self.light_black = map_color(&light_black);
        }

        if let Some(light_red) = cfg.get("light_red") {
            self.light_red = map_color(&light_red);
        }

        if let Some(light_green) = cfg.get("light_green") {
            self.light_green = map_color(&light_green);
        }

        if let Some(light_yellow) = cfg.get("light_yellow") {
            self.light_yellow = map_color(&light_yellow);
        }

        if let Some(light_blue) = cfg.get("light_blue") {
            self.light_blue = map_color(&light_blue);
        }

        if let Some(light_magneta) = cfg.get("light_magneta") {
            self.light_magneta = map_color(&light_magneta);
        }

        if let Some(light_cyan) = cfg.get("light_cyan") {
            self.light_cyan = map_color(&light_cyan);
        }

        if let Some(light_white) = cfg.get("light_white") {
            self.light_white = map_color(&light_white);
        }
    }
}

fn map_color(value: &Value) -> Color {
    match value {
        Value::String(s) => match s.as_str() {
            "Reset" => Color::Reset,
            "Black" => Color::Black,
            "Red" => Color::Red,
            "Green" => Color::Green,
            "Yellow" => Color::Yellow,
            "Blue" => Color::Blue,
            "Magenta" => Color::Magenta,
            "Cyan" => Color::Cyan,
            "Gray" => Color::Gray,
            "DarkGray" => Color::DarkGray,
            "LightRed" => Color::LightRed,
            "LightGreen" => Color::LightGreen,
            "LightYellow" => Color::LightYellow,
            "LightBlue" => Color::LightBlue,
            "LightMagenta" => Color::LightMagenta,
            "LightCyan" => Color::LightCyan,
            "White" => Color::White,
            _ => Color::Reset,
        },
        Value::Integer(i) => Color::Indexed(i.clone() as u8),
        Value::Table(t) => {
            let red = t["red"].as_integer().unwrap().clone() as u8;
            let green = t["green"].as_integer().unwrap().clone() as u8;
            let blue = t["blue"].as_integer().unwrap().clone() as u8;
            Color::Rgb(red, green, blue)
        }
        _ => Color::Reset,
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
            Color::Gray,
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
