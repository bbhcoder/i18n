// i18n-core/src/adapters/memory.rs
use std::sync::Arc;
use crate::types::TranslationEntry;
use crate::store::TranslationStore;
use crate::ast::parse_template;

pub struct MemoryAdapter {
    pub store: TranslationStore,
}

impl MemoryAdapter {
    pub fn new() -> Self {
        Self {
            store: TranslationStore::new(),
        }
    }

    pub fn save(&mut self, entry: TranslationEntry) {
        // کامپایل آنی متن ترجمه به AST و بسته‌بندی در Arc برای دسترسی O(1)
        let compiled_tokens = Arc::new(parse_template(&entry.value));
        self.store
            .entry(entry.locale.clone())
            .or_default()
            .entry(entry.namespace.clone())
            .or_default()
            .insert(entry.key.clone(), compiled_tokens);
    }
}