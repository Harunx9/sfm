use std::collections::HashMap;

use toml::Value;

#[derive(Debug, Clone)]
pub struct FileAssociatedPrograms {
    bindings: HashMap<String, String>,
}

impl Default for FileAssociatedPrograms {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert("default".to_string(), "nvim".to_string());

        FileAssociatedPrograms { bindings }
    }
}

impl FileAssociatedPrograms {
    pub fn update_from_file(&mut self, cfg: &Value) {
        if let Some(file_associated_programs) = cfg.get("file_associated_programs") {
            if let Value::Table(associated_programs_map) = file_associated_programs {
                for (key, val) in associated_programs_map.iter() {
                    self.bindings
                        .insert(key.clone(), val.as_str().unwrap().to_string());
                }
            }
        }
    }
    pub fn get_program_name(&self, file_extension: String) -> String {
        match self.bindings.get(&file_extension) {
            Some(name) => name.clone(),
            None => self.bindings[&"default".to_string()].clone(),
        }
    }
}
