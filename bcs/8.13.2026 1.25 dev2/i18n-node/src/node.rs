// i18n-node/src/node.rs
use std::collections::HashMap;
use std::sync::Arc;
use i18n_core::manager::{I18nManager, CompiledTemplate};
use i18n_core::types::TranslationEntry;

/// ۱. ساختار داده ورودی برای متد add در جاوااسکریپت
#[napi(object)]
pub struct JsTranslationEntry {
    pub locale: String,
    pub namespace: String,
    pub key: String,
    pub value: String,
}

/// ۲. کلاس هندل پیش‌کامپایل‌شده برای رندر فوق‌سريع بدون جستجو (Zero-Lookup)
#[napi]
pub struct JsCompiledTemplate {
    inner: CompiledTemplate,
}

#[napi]
impl JsCompiledTemplate {
    /// سازنده پیش‌فرض جهت ثبت کلاس در V8 و رفع باگ Class contains no constructor
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: CompiledTemplate {
                tokens: Arc::new(vec![]),
                locale: String::new(),
            },
        }
    }

    /// رندر فوری تمپلیت از روی حافظه رم
    #[napi]
    pub fn render(&self, args: Option<HashMap<String, String>>) -> String {
        self.inner.render(&args)
    }
}

/// ۳. کلاس اصلی موتور i18n برای سرورهای Node.js و Nuxt SSR
#[napi]
pub struct I18N {
    inner: I18nManager,
}

#[napi]
impl I18N {
    #[napi(constructor)]
    pub fn new(default_locale: String, fallback_locale: Option<String>) -> Self {
        let fallback = fallback_locale.unwrap_or_else(|| default_locale.clone());
        Self {
            inner: I18nManager::new(&default_locale, &fallback),
        }
    }

    #[napi]
    pub fn set_locale(&mut self, locale: String) {
        self.inner.set_locale(&locale);
    }

    #[napi]
    pub fn set_fallback_locale(&mut self, locale: String) {
        self.inner.set_fallback_locale(&locale);
    }

    #[napi]
    pub fn add(&mut self, entry: JsTranslationEntry) {
        let core_entry = TranslationEntry {
            locale: entry.locale,
            namespace: entry.namespace,
            key: entry.key,
            value: entry.value,
        };
        self.inner.add(core_entry);
    }

    #[napi]
    pub fn add_bundle(&mut self, locale: String, namespace: String, json_string: String) {
        if let Err(err) = self.inner.add_json_bundle(&locale, &namespace, &json_string) {
            eprintln!("[I18N Rust Error]: {}", err);
        }
    }

    #[napi]
    pub fn t(
        &self,
        namespace: String,
        key: String,
        args: Option<HashMap<String, String>>,
    ) -> String {
        self.inner.t(&namespace, &key, args)
    }

    /// متد ساخت هندل کامپایل‌شده (Zero-Lookup)
    #[napi]
    pub fn compile(&self, namespace: String, key: String) -> Option<JsCompiledTemplate> {
        self.inner
            .compile(&namespace, &key)
            .map(|compiled| JsCompiledTemplate { inner: compiled })
    }

    #[napi]
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}