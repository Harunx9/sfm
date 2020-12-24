use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct IconsConfig {
    pub use_icons: bool,
    dir_icons: HashMap<String, String>,
    files_icon: HashMap<String, String>,
}

impl Default for IconsConfig {
    fn default() -> Self {
        IconsConfig {
            use_icons: true,
            dir_icons: get_default_dir_icons(),
            files_icon: get_default_files_icons(),
        }
    }
}

fn get_default_dir_icons() -> HashMap<String, String> {
    let mut icon_map = HashMap::new();
    icon_map.insert(".git".to_string(), "".to_string());
    icon_map.insert("node_modules".to_string(), "".to_string());
    icon_map.insert("default".to_string(), "".to_string());

    icon_map
}

fn get_default_files_icons() -> HashMap<String, String> {
    let mut icon_map = HashMap::new();
    //GIT
    icon_map.insert(".gitignore".to_string(), "".to_string());
    icon_map.insert(".gitmodules".to_string(), "".to_string());

    //PROGRAMMING LANGUAGES
    icon_map.insert("rs".to_string(), "".to_string());
    icon_map.insert("cs".to_string(), "".to_string());
    icon_map.insert("cpp".to_string(), "ﭱ".to_string());
    icon_map.insert("c".to_string(), "".to_string());
    icon_map.insert("hpp".to_string(), "".to_string());
    icon_map.insert("h".to_string(), "".to_string());
    icon_map.insert("js".to_string(), "".to_string());
    icon_map.insert("ts".to_string(), "".to_string());
    icon_map.insert("jsx".to_string(), "".to_string());
    icon_map.insert("tsx".to_string(), "ﰆ".to_string());
    icon_map.insert("html".to_string(), "".to_string());
    icon_map.insert("css".to_string(), "".to_string());
    icon_map.insert("sass".to_string(), "".to_string());
    icon_map.insert("toml".to_string(), "".to_string());
    icon_map.insert("yaml".to_string(), "".to_string());
    icon_map.insert("php".to_string(), "".to_string());
    icon_map.insert("py".to_string(), "".to_string());
    icon_map.insert("rb".to_string(), "".to_string());
    icon_map.insert("java".to_string(), "".to_string());
    icon_map.insert("lock".to_string(), "".to_string());
    icon_map.insert("default".to_string(), "".to_string());

    icon_map
}

impl IconsConfig {
    pub fn get_dir_icon(&self, dir_name: String) -> String {
        match self.dir_icons.get(&dir_name) {
            Some(icon) => icon.clone(),
            None => self.dir_icons["default"].clone(),
        }
    }

    pub fn get_file_icon(&self, file_name: String) -> String {
        match self.files_icon.get(&file_name) {
            Some(icon) => icon.clone(),
            None => self.files_icon["default"].clone(),
        }
    }
}
