use serde::{Serialize, Deserialize};

pub type Locale = String;
pub type Namespace = String;
pub type Key = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct TranslationEntry {
    pub locale: Locale,
    pub namespace: Namespace,
    pub key: Key,
    pub value: String,
}
