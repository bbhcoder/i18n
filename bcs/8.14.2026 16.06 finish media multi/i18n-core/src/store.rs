// i18n-core/src/store.rs
use std::collections::HashMap;
use std::sync::Arc;
use crate::ast::Token;

/// دیتابیس پرسرعت ما حالا از Arc برای دسترسی مستقیم و بدون کپی (Zero-Copy) استفاده می‌کند
pub type TranslationStore =
    HashMap<String, HashMap<String, HashMap<String, Arc<Vec<Token>>>>>;