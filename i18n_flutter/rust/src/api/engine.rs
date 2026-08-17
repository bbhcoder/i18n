// i18n_flutter/rust/src/api/engine.rs

use std::collections::HashMap;
use i18n_core::manager::I18nManager; // اتصال به هسته اصلی[cite: 1]
use flutter_rust_bridge::frb;

// FRB به صورت خودکار این Struct رو تبدیل به یه کلاس Dart می‌کنه!
pub struct I18nEngine {
    inner: I18nManager,
}

impl I18nEngine {
    // تگ sync باعث میشه این متد تو فلاتر بدون نیاز به await و آنی اجرا بشه
    #[frb(sync)]
    pub fn new(default_locale: String, fallback_locale: Option<String>) -> Self {
        let fallback = fallback_locale.unwrap_or_else(|| default_locale.clone());
        Self {
            inner: I18nManager::new(&default_locale, &fallback), //[cite: 1]
        }
    }

    #[frb(sync)]
    pub fn set_locale(&mut self, locale: String) {
        self.inner.set_locale(&locale); //[cite: 1]
    }

    // اگر خطایی رخ بده، FRB خودش اون رو تو فلاتر به عنوان Exception پرت می‌کنه
    #[frb(sync)]
    pub fn add_bundle(&mut self, locale: String, namespace: String, json_string: String) -> Result<(), String> {
        self.inner.add_json_bundle(&locale, &namespace, &json_string) //[cite: 1]
    }

    #[frb(sync)]
    pub fn t(&self, namespace: String, key: String, args_json: Option<String>) -> String {
        // برای شروع، آرگومان‌ها رو با JSON پاس می‌دیم (مثل نسخه اولیه Wasm)
        let args: Option<HashMap<String, String>> = args_json.and_then(|json_str| {
            serde_json::from_str(&json_str).ok()
        });
        self.inner.t(&namespace, &key, args) //[cite: 1]
    }

    #[frb(sync)]
    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.inner.set_debug_mode(enabled);
    }

    #[frb(sync)]
    pub fn diagnostics(&self) -> String {
        self.inner.diagnostics()
    }
}