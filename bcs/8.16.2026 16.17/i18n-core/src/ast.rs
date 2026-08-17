// i18n-core/src/ast.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    /// متن عادی (بدون نیاز به هیچ پردازشی در زمان اجرا)
    Text(String),
    /// متغیر ساده مثل {name}
    Var(String),
    /// شرط جمع و مفرد مثل {unread: 0=هیچ | 1=یکی | _=# تا}
    Plural {
        var: String,
        rules: Vec<(String, String)>,
        default: String,
    },
}

/// تابع پیش‌کامپایل: متن خام ترجمه را به توکن‌های AST تبدیل می‌کند
pub fn parse_template(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            // پشتیبانی از Escape Character (مثلاً \{ برای چاپ آکولاد عادی)
            if let Some(&next_c) = chars.peek() {
                if next_c == '{' || next_c == '}' || next_c == '\\' {
                    buffer.push(next_c);
                    chars.next();
                    continue;
                }
            }
            buffer.push(c);
        } else if c == '{' {
            if !buffer.is_empty() {
                tokens.push(Token::Text(buffer.clone()));
                buffer.clear();
            }

            // استخراج محتوای داخل آکولاد
            let mut inside = String::new();
            while let Some(inner_c) = chars.next() {
                if inner_c == '}' {
                    break;
                }
                inside.push(inner_c);
            }

            let trimmed = inside.trim();
            if let Some(colon_idx) = trimmed.find(':') {
                // پارس کردن قوانین Plural
                let var_name = trimmed[..colon_idx].trim().to_string();
                let rules_str = &trimmed[colon_idx + 1..];
                let mut rules = Vec::new();
                let mut default_val = String::new();

                for rule in rules_str.split('|') {
                    let rule = rule.trim();
                    if let Some(eq_idx) = rule.find('=') {
                        let cond = rule[..eq_idx].trim().to_string();
                        let val = rule[eq_idx + 1..].trim().to_string();
                        if cond == "_" {
                            default_val = val;
                        } else {
                            rules.push((cond, val));
                        }
                    }
                }
                tokens.push(Token::Plural {
                    var: var_name,
                    rules,
                    default: default_val,
                });
            } else {
                tokens.push(Token::Var(trimmed.to_string()));
            }
        } else {
            buffer.push(c);
        }
    }

    if !buffer.is_empty() {
        tokens.push(Token::Text(buffer));
    }

    tokens
}