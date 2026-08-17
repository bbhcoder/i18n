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
        let compiled_tokens = Arc::new(parse_template(&entry.value));
        
        let composite_key = format!("{}:{}:{}", entry.locale, entry.namespace, entry.key);
        
        self.store.insert(composite_key, compiled_tokens);
    }
}