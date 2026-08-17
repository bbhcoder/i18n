// I18nEngine-node/src/node.rs
use std::collections::HashMap;
use std::sync::Arc;
use i18n_core::manager::{I18nManager, CompiledTemplate};
use i18n_core::types::TranslationEntry;
use std::fs;

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

/// ۳. کلاس اصلی موتور I18nEngine برای سرورهای Node.js و Nuxt SSR
#[napi]
pub struct I18nEngine {
    inner: I18nManager,
}

#[napi]
impl I18nEngine {
    #[napi(constructor)]
    pub fn new(default_locale: String, fallback_locale: Option<String>) -> Self {
        let fallback = fallback_locale.unwrap_or_else(|| default_locale.clone());
        Self {
            inner: I18nManager::new(&default_locale, &fallback),
        }
    }

    #[napi]
    pub fn help(&self) -> String {
        self.inner.help()
    }

    #[napi]
    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.inner.set_debug_mode(enabled);
    }

    #[napi]
    pub fn diagnostics(&self) -> String {
        self.inner.diagnostics()
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
            eprintln!("[I18nEngine Rust Error]: {}", err);
        }
    }

    #[napi]
    pub fn t(
        &self,
        namespace: String,
        key: String,
        args_json: Option<String>,
    ) -> String {
        let args: Option<HashMap<String, String>> = args_json.and_then(|json_str| {
            serde_json::from_str(&json_str).ok()
        });
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

    #[napi]
    pub fn add_media(&mut self, locale: String, namespace: String, key: String, url: String) {
        self.inner.add_media(&locale, &namespace, &key, &url);
    }

    /// دریافت فایل مدیا با توجه به زبان فعلی
    #[napi]
    pub fn get_media(&self, namespace: String, key: String) -> Option<String> {
        self.inner.get_media(&namespace, &key)
    }

    /// اسکن خودکار یک دایرکتوری و تخصیص مدیا بر اساس نام فایل (سناریو دوم)
    /// مسیر دایرکتوری باید به شکل زیر باشد: root_path / {fa|en} / file.ext
    #[napi]
    pub fn scan_media_folder(&mut self, namespace: String, root_path: String, base_url: String) -> Result<(), napi::Error> {
        let entries = fs::read_dir(&root_path)
            .map_err(|e| napi::Error::from_reason(format!("Failed to read root directory: {}", e)))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // نام فولدر همان مخفف زبان است (مثلا fa یا en)
                if let Some(locale_os) = path.file_name() {
                    let locale = locale_os.to_string_lossy().to_string();
                    
                    if let Ok(files) = fs::read_dir(&path) {
                        for file in files.flatten() {
                            let file_path = file.path();
                            if file_path.is_file() {
                                if let Some(file_name_os) = file_path.file_name() {
                                    let file_name = file_name_os.to_string_lossy().to_string();
                                    
                                    // اسم فایل بدون پسوند به عنوان Key در نظر گرفته می‌شود
                                    // مثلا logo.png می‌شود کلید logo
                                    let key = file_path.file_stem()
                                        .map(|s| s.to_string_lossy().to_string())
                                        .unwrap_or_else(|| file_name.clone());

                                    // تولید آدرس نهایی دسترسی به فایل
                                    let url = format!("{}/{}/{}", base_url, locale, file_name);
                                    
                                    self.inner.add_media(&locale, &namespace, &key, &url);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}