use std::collections::HashMap;

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
    pub fn get_program_name(&self, file_extension: String) -> String {
        match self.bindings.get(&file_extension) {
            Some(name) => name.clone(),
            None => self.bindings[&"default".to_string()].clone(),
        }
    }
}
