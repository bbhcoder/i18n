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

/// تعیین دسته‌بندی جمع بر اساس کامل‌ترین استاندارد CLDR
pub fn get_plural_category(locale: &str, n: f64) -> PluralCategory {
    // استخراج پیشوند زبان (مثلاً "en-US" میشه "en")
    let lang = locale.split('-').next().unwrap_or(locale).to_lowercase();
    
    let i = n.abs() as i64;
    let is_int = n.fract() == 0.0;
    
    // متغیرهای پرتکرار برای زبان‌های پیچیده
    let mod10 = i % 10;
    let mod100 = i % 100;

    match lang.as_str() {
        // ---------------------------------------------------------
        // گروه ۱: بدون جمع (خانواده‌های آسیایی و فارسی)
        // از نظر CLDR این زبان‌ها فقط Other دارند، اما برای راحتی کار دولوپرها Zero و One جدا شده است
        // ---------------------------------------------------------
        "fa" | "ja" | "ko" | "zh" | "vi" | "th" | "id" | "ms" | "my" | "km" | "lo" | "dz" | "ig" | "yo" => {
            if is_int && i == 0 { PluralCategory::Zero }
            else if is_int && i == 1 { PluralCategory::One }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۲: خانواده ژرمن و رومنس (انگلیسی، آلمانی، اسپانیایی و...)
        // قانون: فقط عدد ۱ مفرد است.
        // ---------------------------------------------------------
        "en" | "de" | "es" | "it" | "nl" | "sv" | "da" | "no" | "nn" | "nb" | "bg" | "el" | "fi" | "hu" | "et" | "ca" | "eo" | "fo" | "gl" | "sw" | "ur" => {
            if is_int && i == 0 { PluralCategory::Zero }
            else if is_int && i == 1 { PluralCategory::One }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۳: فرانسوی، پرتغالی برزیل، هندی و امهری
        // قانون: اعداد بین صفر تا دو (حتی اعشاری) مفرد محسوب می‌شوند.
        // ---------------------------------------------------------
        "fr" | "pt-br" | "hi" | "am" | "bn" | "gu" | "mr" | "pa" | "zu" => {
            if n >= 0.0 && n < 2.0 { PluralCategory::One }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۴: عربی (پیچیده‌ترین سیستم جمع با ۶ حالت کامل)
        // ---------------------------------------------------------
        "ar" => {
            if is_int && i == 0 { PluralCategory::Zero }
            else if is_int && i == 1 { PluralCategory::One }
            else {
                match mod100 {
                    2 if is_int => PluralCategory::Two,
                    3..=10 if is_int => PluralCategory::Few,
                    11..=99 if is_int => PluralCategory::Many,
                    _ => PluralCategory::Other,
                }
            }
        }

        // ---------------------------------------------------------
        // گروه ۵: زبان‌های اسلاوی شرقی (روسی، اوکراینی، بلاروسی)
        // قانون: اعدادی که به ۱ ختم شوند (جز ۱۱) مفردند.
        // ---------------------------------------------------------
        "ru" | "uk" | "be" => {
            if is_int && mod10 == 1 && mod100 != 11 { PluralCategory::One }
            else if is_int && (2..=4).contains(&mod10) && !(12..=14).contains(&mod100) { PluralCategory::Few }
            else if is_int && (mod10 == 0 || (5..=9).contains(&mod10) || (11..=14).contains(&mod100)) { PluralCategory::Many }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۶: لهستانی
        // قانون: دقیقاً مثل روسی است، اما عدد ۱ فقط مفرد است (نه اعدادی که به ۱ ختم می‌شوند)
        // ---------------------------------------------------------
        "pl" => {
            if is_int && i == 1 { PluralCategory::One }
            else if is_int && (2..=4).contains(&mod10) && !(12..=14).contains(&mod100) { PluralCategory::Few }
            else if is_int && (mod10 == 0 || (5..=9).contains(&mod10) || (11..=14).contains(&mod100)) { PluralCategory::Many }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۷: زبان‌های اسلاوی غربی (چکی، اسلواک)
        // ---------------------------------------------------------
        "cs" | "sk" => {
            if is_int && i == 1 { PluralCategory::One }
            else if is_int && (2..=4).contains(&i) { PluralCategory::Few }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۸: رومانیایی و مولداویایی
        // ---------------------------------------------------------
        "ro" | "mo" => {
            if is_int && i == 1 { PluralCategory::One }
            else if is_int && (i == 0 || (1..=19).contains(&mod100)) { PluralCategory::Few }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۹: کرواتی، صربی، بوسنیایی
        // ---------------------------------------------------------
        "hr" | "sr" | "bs" | "sh" => {
            if is_int && mod10 == 1 && mod100 != 11 { PluralCategory::One }
            else if is_int && (2..=4).contains(&mod10) && !(12..=14).contains(&mod100) { PluralCategory::Few }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۱۰: اسلوونیایی
        // ---------------------------------------------------------
        "sl" => {
            if is_int && mod100 == 1 { PluralCategory::One }
            else if is_int && mod100 == 2 { PluralCategory::Two }
            else if is_int && (3..=4).contains(&mod100) { PluralCategory::Few }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۱۱: عبری
        // ---------------------------------------------------------
        "he" | "iw" => {
            if is_int && i == 1 { PluralCategory::One }
            else if is_int && i == 2 { PluralCategory::Two }
            else if is_int && i % 10 == 0 && i != 0 { PluralCategory::Many }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۱۲: لیتوانیایی
        // ---------------------------------------------------------
        "lt" => {
            if is_int && mod10 == 1 && mod100 != 11 { PluralCategory::One }
            else if is_int && (2..=9).contains(&mod10) && !(11..=19).contains(&mod100) { PluralCategory::Few }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۱۳: لتونیایی
        // ---------------------------------------------------------
        "lv" => {
            if is_int && mod10 == 1 && mod100 != 11 { PluralCategory::One }
            else if is_int && (mod10 == 0 || (11..=19).contains(&mod100)) { PluralCategory::Zero }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۱۴: ولزی
        // ---------------------------------------------------------
        "cy" => {
            if is_int && i == 0 { PluralCategory::Zero }
            else if is_int && i == 1 { PluralCategory::One }
            else if is_int && i == 2 { PluralCategory::Two }
            else if is_int && i == 3 { PluralCategory::Few }
            else if is_int && i == 6 { PluralCategory::Many }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۱۵: ایرلندی
        // ---------------------------------------------------------
        "ga" => {
            if is_int && i == 1 { PluralCategory::One }
            else if is_int && i == 2 { PluralCategory::Two }
            else if is_int && (3..=6).contains(&i) { PluralCategory::Few }
            else if is_int && (7..=10).contains(&i) { PluralCategory::Many }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // گروه ۱۶: مقدونی و ایسلندی
        // ---------------------------------------------------------
        "is" | "mk" => {
            if is_int && mod10 == 1 && mod100 != 11 { PluralCategory::One }
            else { PluralCategory::Other }
        }

        // ---------------------------------------------------------
        // Fallback: حالت پیش‌فرض برای هر زبان ناشناخته‌ی دیگر
        // ---------------------------------------------------------
        _ => {
            if is_int && i == 1 { PluralCategory::One }
            else { PluralCategory::Other }
        }
    }
}