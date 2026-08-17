// i18n-core/src/manager.rs
use std::collections::HashMap;
use std::sync::Arc;
use crate::types::TranslationEntry;
use crate::store::TranslationStore;
use crate::adapters::memory::MemoryAdapter;
use crate::ast::Token;
use crate::plurals::get_plural_category;
use serde_json::Value;

/// هندل کامپایل‌شده برای رندرینگ فوق‌سریع بدون نیاز به جستجو در دیتابیس (Zero-Lookup)
#[derive(Clone)]
pub struct CompiledTemplate {
    pub tokens: Arc<Vec<Token>>,
    pub locale: String,
}

impl CompiledTemplate {
    pub fn render(&self, args: &Option<HashMap<String, String>>) -> String {
        I18nManager::render_tokens(&self.tokens, args, &self.locale)
    }
}

pub struct I18nManager {
    pub store: TranslationStore,
    pub current_locale: String,
    pub fallback_locale: String,
    pub memory: MemoryAdapter,
}

impl I18nManager {
    pub fn new(default_locale: &str, fallback_locale: &str) -> Self {
        Self {
            store: TranslationStore::new(),
            current_locale: default_locale.to_string(),
            fallback_locale: fallback_locale.to_string(),
            memory: MemoryAdapter::new(),
        }
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.current_locale = locale.to_string();
    }

    pub fn set_fallback_locale(&mut self, locale: &str) {
        self.fallback_locale = locale.to_string();
    }

    pub fn add(&mut self, entry: TranslationEntry) {
        self.memory.save(entry.clone());
        self.store = self.memory.store.clone();
    }

    fn flatten_json(prefix: &str, value: &Value, output: &mut Vec<(String, String)>) {
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    Self::flatten_json(&new_prefix, v, output);
                }
            }
            Value::String(s) => {
                output.push((prefix.to_string(), s.clone()));
            }
            other => {
                output.push((prefix.to_string(), other.to_string()));
            }
        }
    }

    pub fn add_json_bundle(&mut self, locale: &str, namespace: &str, json_str: &str) -> Result<(), String> {
        let parsed: Value = serde_json::from_str(json_str)
            .map_err(|e| format!("Invalid JSON bundle: {}", e))?;
        let mut flat_entries = Vec::new();
        Self::flatten_json("", &parsed, &mut flat_entries);
        for (key, value) in flat_entries {
            self.add(TranslationEntry {
                locale: locale.to_string(),
                namespace: namespace.to_string(),
                key,
                value,
            });
        }
        Ok(())
    }

    fn lookup(&self, locale: &str, namespace: &str, key: &str) -> Option<&Arc<Vec<Token>>> {
        self.store
            .get(locale)
            .and_then(|ns| ns.get(namespace))
            .and_then(|keys| keys.get(key))
    }

    /// ایجاد هندل کش‌شده برای سرویس‌های پرفشار (مثل ارسال پیامک انبوه)
    pub fn compile(&self, namespace: &str, key: &str) -> Option<CompiledTemplate> {
        if let Some(tokens) = self.lookup(&self.current_locale, namespace, key) {
            return Some(CompiledTemplate {
                tokens: Arc::clone(tokens),
                locale: self.current_locale.clone(),
            });
        }
        if let Some(tokens) = self.lookup(&self.fallback_locale, namespace, key) {
            return Some(CompiledTemplate {
                tokens: Arc::clone(tokens),
                locale: self.fallback_locale.clone(),
            });
        }
        None
    }

    pub fn render_tokens(
        tokens: &[Token],
        args: &Option<HashMap<String, String>>,
        locale: &str,
    ) -> String {
        let mut result = String::with_capacity(64);
        for token in tokens {
            match token {
                Token::Text(text) => {
                    result.push_str(text);
                }
                Token::Var(var_name) => {
                    let val = args
                        .as_ref()
                        .and_then(|map| map.get(var_name))
                        .cloned()
                        .unwrap_or_else(|| format!("{{{}}}", var_name));
                    result.push_str(&val);
                }
                Token::Plural { var, rules, default } => {
                    let val_str = args
                        .as_ref()
                        .and_then(|map| map.get(var))
                        .cloned()
                        .unwrap_or_else(|| "0".to_string());
                    let num_val: f64 = val_str.parse().unwrap_or(0.0);
                    let category = get_plural_category(locale, num_val).as_str();
                    let mut exact_match = None;
                    let mut category_match = None;
                    for (cond, text) in rules {
                        if cond == &val_str {
                            exact_match = Some(text.as_str());
                            break;
                        }
                        if cond == category && category_match.is_none() {
                            category_match = Some(text.as_str());
                        }
                    }
                    let selected = exact_match
                        .or(category_match)
                        .unwrap_or(default.as_str());
                    if selected.contains('#') {
                        result.push_str(&selected.replace('#', &val_str));
                    } else {
                        result.push_str(selected);
                    }
                }
            }
        }
        result
    }

    pub fn t(
        &self,
        namespace: &str,
        key: &str,
        args: Option<HashMap<String, String>>,
    ) -> String {
        // ۱. جستجوی مستقیم کلید
        if let Some(tokens) = self.lookup(&self.current_locale, namespace, key) {
            return Self::render_tokens(tokens, &args, &self.current_locale);
        }
        if let Some(tokens) = self.lookup(&self.fallback_locale, namespace, key) {
            return Self::render_tokens(tokens, &args, &self.fallback_locale);         
        }

        // ۲. هوش خودکار Pluralization بر اساس متغیر count (استاندارد CLDR)
        if let Some(ref map) = args {
            if let Some(count_str) = map.get("count") {
                let count_val: f64 = count_str.parse().unwrap_or(0.0);

                // بررسی در زبان فعلی (مثلاً cart.item_one یا cart.item_other)
                let cat = get_plural_category(&self.current_locale, count_val).as_str();
                let plural_key = format!("{}_{}", key, cat);
                let other_key = format!("{}_other", key);

                if let Some(tokens) = self.lookup(&self.current_locale, namespace, &plural_key)
                    .or_else(|| self.lookup(&self.current_locale, namespace, &other_key))
                {
                    return Self::render_tokens(tokens, &args, &self.current_locale);
                }

                // بررسی در زبان Fallback
                let cat_fb = get_plural_category(&self.fallback_locale, count_val).as_str();
                let plural_key_fb = format!("{}_{}", key, cat_fb);
                let other_key_fb = format!("{}_other", key);

                if let Some(tokens) = self.lookup(&self.fallback_locale, namespace, &plural_key_fb)
                    .or_else(|| self.lookup(&self.fallback_locale, namespace, &other_key_fb))
                {
                    return Self::render_tokens(tokens, &args, &self.fallback_locale);
                }
            }
        }

        format!("{}.{}", namespace, key)
    }
}