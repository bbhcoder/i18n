// i18n-core/src/adapters/file.rs

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

    // خروجی تابع به جای Value خام، به Result تبدیل شد
    pub fn load(&self, locale: &str) -> Result<Value, String> {
        let path = format!("{}/{}.json", self.base_path, locale);
        
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read language file '{}': {}", path, e))?;
            
        let parsed = serde_json::from_str(&content)
            .map_err(|e| format!("Invalid JSON format in file '{}': {}", path, e))?;
            
        Ok(parsed)
    }

    // خروجی به Result تغییر کرد
    pub fn save(&self, locale: &str, data: &Value) -> Result<(), String> {
        let path = format!("{}/{}.json", self.base_path, locale);
        
        let json_str = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to stringify JSON for '{}': {}", path, e))?;
            
        fs::write(&path, json_str)
            .map_err(|e| format!("Failed to write language file '{}': {}", path, e))?;
            
        Ok(())
    }
}