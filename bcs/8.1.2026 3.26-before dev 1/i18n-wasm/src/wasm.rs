// i18n-wasm/src/wasm.rs
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use i18n_core::manager::I18nManager;
use i18n_core::types::TranslationEntry;

#[wasm_bindgen]
pub struct I18nWasm {
    inner: I18nManager,
}

#[wasm_bindgen]
impl I18nWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(default_locale: String, fallback_locale: Option<String>) -> I18nWasm {
        let fallback = fallback_locale.unwrap_or_else(|| default_locale.clone());
        I18nWasm {
            inner: I18nManager::new(&default_locale, &fallback),
        }
    }

    #[wasm_bindgen]
    pub fn set_locale(&mut self, locale: String) {
        self.inner.set_locale(&locale);
    }

    #[wasm_bindgen]
    pub fn set_fallback_locale(&mut self, locale: String) {
        self.inner.set_fallback_locale(&locale);
    }

    #[wasm_bindgen]
    pub fn add(
        &mut self,
        locale: String,
        namespace: String,
        key: String,
        value: String,
    ) {
        let entry = TranslationEntry {
            locale,
            namespace,
            key,
            value,
        };
        self.inner.add(entry);
    }

    #[wasm_bindgen]
    pub fn add_bundle(&mut self, locale: String, namespace: String, json_string: String) {
        if let Err(err) = self.inner.add_json_bundle(&locale, &namespace, &json_string) {
            // در محیط WebAssembly خطا را در کنسول جاوااسکریپت/مرورگر چاپ می‌کنیم
            web_sys::console::error_1(&format!("[I18N Wasm Error]: {}", err).into());
        }
    }

    #[wasm_bindgen]
    pub fn t(&self, namespace: String, key: String, args_json: Option<String>) -> String {
        // در WebAssembly برای راحتی کار با جاوااسکریپت، آرگومان‌ها را به صورت رشته JSON می‌گیریم و تبدیل می‌کنیم
        let args: Option<HashMap<String, String>> = args_json.and_then(|json_str| {
            serde_json::from_str(&json_str).ok()
        });

        self.inner.t(&namespace, &key, args)
    }
}