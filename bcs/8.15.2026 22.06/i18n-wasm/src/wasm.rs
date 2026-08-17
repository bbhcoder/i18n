// i18n-wasm/src/wasm.rs
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use i18n_core::manager::{I18nManager, CompiledTemplate};
use i18n_core::types::TranslationEntry;

/// هندل وب‌اسمبلی برای رندرینگ فوق‌سريع در مرورگر
#[wasm_bindgen]
pub struct WasmCompiledTemplate {
    inner: CompiledTemplate,
}

#[wasm_bindgen]
impl WasmCompiledTemplate {
    #[wasm_bindgen]
    pub fn render(&self, args_json: Option<String>) -> String {
        let args: Option<HashMap<String, String>> = args_json.and_then(|json_str| {
            serde_json::from_str(&json_str).ok()
        });
        self.inner.render(&args)
    }
}

#[wasm_bindgen]
pub struct I18nEngine {
    inner: I18nManager,
}

#[wasm_bindgen]
impl I18nEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(default_locale: String, fallback_locale: Option<String>) -> I18nEngine {
        let fallback = fallback_locale.unwrap_or_else(|| default_locale.clone());
        I18nEngine {
            inner: I18nManager::new(&default_locale, &fallback),
        }
    }

    #[wasm_bindgen(js_name = setDebugMode)]
    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.inner.set_debug_mode(enabled);
    }

    #[wasm_bindgen]
    pub fn diagnostics(&self) -> String {
        self.inner.diagnostics()
    }

    #[wasm_bindgen(js_name = setLocale)]
    pub fn set_locale(&mut self, locale: String) {
        self.inner.set_locale(&locale);
    }

    #[wasm_bindgen(js_name = setFallbackLocale)]
    pub fn set_fallback_locale(&mut self, locale: String) {
        self.inner.set_fallback_locale(&locale);
    }

    #[wasm_bindgen(js_name = addBundle)]
    pub fn add_bundle(&mut self, locale: String, namespace: String, json_string: String) {
        if let Err(err) = self.inner.add_json_bundle(&locale, &namespace, &json_string) {
            web_sys::console::error_1(&format!("[I18N Wasm Error]: {}", err).into());
        }
    }

    #[wasm_bindgen(js_name = addMedia)]
    pub fn add_media(&mut self, locale: String, namespace: String, key: String, url: String) {
        self.inner.add_media(&locale, &namespace, &key, &url);
    }

    #[wasm_bindgen(js_name = getMedia)]
    pub fn get_media(&self, namespace: String, key: String) -> Option<String> {
        self.inner.get_media(&namespace, &key)
    }

    // متدهای t، compile و version نیازی به تغییر ندارند چون نامشان تک‌کلمه‌ای است
    #[wasm_bindgen]
    pub fn t(&self, namespace: String, key: String, args_json: Option<String>) -> String {
        let args: Option<HashMap<String, String>> = args_json.and_then(|json_str| {
            serde_json::from_str(&json_str).ok()
        });
        self.inner.t(&namespace, &key, args)
    }

    #[wasm_bindgen]
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}