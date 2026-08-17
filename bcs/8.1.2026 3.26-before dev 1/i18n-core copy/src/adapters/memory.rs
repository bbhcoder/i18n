use crate::types::TranslationEntry;
use crate::store::TranslationStore;

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
        self.store
            .entry(entry.locale.clone())
            .or_default()
            .entry(entry.namespace.clone())
            .or_default()
            .insert(entry.key.clone(), entry.value.clone());
    }
}
