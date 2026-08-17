use std::fs;
use serde_json::Value;

pub struct FileAdapter {
    pub base_path: String,
}

impl FileAdapter {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }

    pub fn load(&self, locale: &str) -> Value {
        let path = format!("{}/{}.json", self.base_path, locale);
        let content = fs::read_to_string(path).unwrap();
        serde_json::from_str(&content).unwrap()
    }

    pub fn save(&self, locale: &str, data: &Value) {
        let path = format!("{}/{}.json", self.base_path, locale);
        fs::write(path, serde_json::to_string_pretty(data).unwrap()).unwrap();
    }
}
