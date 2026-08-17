// i18n-core/src/store.rs
use std::collections::HashMap;
use std::sync::Arc;
use crate::ast::Token;

pub type TranslationStore = HashMap<String, Arc<Vec<Token>>>;