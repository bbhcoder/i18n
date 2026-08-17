// i18n-core/src/plurals.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluralCategory {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

impl PluralCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            PluralCategory::Zero => "zero",
            PluralCategory::One => "one",
            PluralCategory::Two => "two",
            PluralCategory::Few => "few",
            PluralCategory::Many => "many",
            PluralCategory::Other => "other",
        }
    }
}

/// تشخیص دسته‌بندی CLDR بر اساس زبان و مقدار عدد
pub fn get_plural_category(locale: &str, n: f64) -> PluralCategory {
    let lang = locale.split('-').next().unwrap_or(locale).to_lowercase();
    let i = n.abs() as i64;
    let v = n.fract(); // باقیمانده اعشاری

    match lang.as_str() {
        // فارسی، ژاپنی، ترکی، کره‌ای (یا بدون جمع ساختاری، یا ساده)
        "fa" | "ja" | "ko" | "tr" | "zh" => {
            if i == 0 && v == 0.0 {
                PluralCategory::Zero
            } else if i == 1 && v == 0.0 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        }
        // انگلیسی، اسپانیایی، فرانسوی، آلمانی، ایتالیایی
        "en" | "es" | "fr" | "de" | "it" | "pt" => {
            if i == 0 && v == 0.0 {
                PluralCategory::Zero
            } else if (i == 0 || i == 1) && lang == "fr" {
                PluralCategory::One
            } else if i == 1 && v == 0.0 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        }
        // عربی (پشتیبانی از ۶ حالت کامل یونیکد)
        "ar" => {
            if n == 0.0 {
                PluralCategory::Zero
            } else if n == 1.0 {
                PluralCategory::One
            } else if n == 2.0 {
                PluralCategory::Two
            } else if (3..=10).contains(&(i % 100)) && v == 0.0 {
                PluralCategory::Few
            } else if (11..=99).contains(&(i % 100)) && v == 0.0 {
                PluralCategory::Many
            } else {
                PluralCategory::Other
            }
        }
        // روسی، اوکراینی، لهستانی (قوانین باقیمانده Modulo)
        "ru" | "uk" | "be" | "pl" => {
            let mod10 = i % 10;
            let mod100 = i % 100;
            if mod10 == 1 && mod100 != 11 {
                PluralCategory::One
            } else if (2..=4).contains(&mod10) && !(12..=14).contains(&mod100) {
                PluralCategory::Few
            } else if mod10 == 0
                || (5..=9).contains(&mod10)
                || (11..=14).contains(&mod100)
            {
                PluralCategory::Many
            } else {
                PluralCategory::Other
            }
        }
        // پیش‌فرض جهانی برای سایر زبان‌ها
        _ => {
            if n == 1.0 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        }
    }
}