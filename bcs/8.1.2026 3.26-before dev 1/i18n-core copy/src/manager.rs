// i18n-core/src/manager.rs
use std::collections::HashMap;
use crate::types::TranslationEntry;
use crate::store::TranslationStore;
use crate::adapters::memory::MemoryAdapter;
use serde_json::Value;

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

    fn lookup(&self, locale: &str, namespace: &str, key: &str) -> Option<String> {
        self.store
            .get(locale)
            .and_then(|ns| ns.get(namespace))
            .and_then(|keys| keys.get(key))
            .cloned()
    }

    // --- موتور جدید: پردازش همزمان متغیرها و برش‌دهی درون‌خطی (Inline Plurals) ---
    fn render_template(text: &str, args: &Option<HashMap<String, String>>) -> String {
        let mut result = String::new();
        let mut rest = text;

        while let Some(start_idx) = rest.find('{') {
            // ۱. افزودن متن قبل از آکولاد باز
            result.push_str(&rest[..start_idx]);
            rest = &rest[start_idx + 1..];

            // ۲. پیدا کردن آکولاد بسته
            if let Some(end_idx) = rest.find('}') {
                let inside = &rest[..end_idx].trim();
                rest = &rest[end_idx + 1..];

                // ۳. بررسی اینکه آیا سینتکس درون‌خطی (دارای دوگانه :) است یا متغیر ساده
                if let Some(colon_idx) = inside.find(':') {
                    // --- حالت Plural درون‌خطی: {unread: 0=هیچ|1=یکی|_=# تا} ---
                    let var_name = inside[..colon_idx].trim();
                    let rules_str = &inside[colon_idx + 1..];

                    let var_value = args
                        .as_ref()
                        .and_then(|map| map.get(var_name))
                        .cloned()
                        .unwrap_or_else(|| "0".to_string());

                    let mut matched_text = None;
                    let mut default_text = None;

                    // اسکن کردن شرط‌های جداشده با |
                    for rule in rules_str.split('|') {
                        let rule = rule.trim();
                        if let Some(eq_idx) = rule.find('=') {
                            let cond = rule[..eq_idx].trim();
                            let val_text = rule[eq_idx + 1..].trim();

                            if cond == var_value {
                                matched_text = Some(val_text.to_string());
                                break;
                            } else if cond == "_" {
                                default_text = Some(val_text.to_string());
                            }
                        }
                    }

                    // انتخاب متن شرط صحیح یا پیش‌فرض، و جایگزینی کاراکتر # با عدد
                    let final_text = matched_text
                        .or(default_text)
                        .unwrap_or_default()
                        .replace('#', &var_value);

                    result.push_str(&final_text);
                } else {
                    // --- حالت متغیر ساده: {name} ---
                    let var_value = args
                        .as_ref()
                        .and_then(|map| map.get(*inside))
                        .cloned()
                        .unwrap_or_else(|| format!("{{{}}}", inside));

                    result.push_str(&var_value);
                }
            } else {
                // اگر آکولاد بسته نداشت، کل متن باقیمانده را برمی‌گردانیم
                result.push('{');
                break;
            }
        }

        result.push_str(rest);
        result
    }

    pub fn t(
        &self,
        namespace: &str,
        key: &str,
        args: Option<HashMap<String, String>>,
    ) -> String {
        let raw_value = self
            .lookup(&self.current_locale, namespace, key)
            .or_else(|| self.lookup(&self.fallback_locale, namespace, key))
            .unwrap_or_else(|| format!("{}.{}", namespace, key));

        Self::render_template(&raw_value, &args)
    }
}