use wasm_bindgen::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use geo_core::store::GeoStore;
use geo_core::types::{Country, Province, City};

#[wasm_bindgen]
pub struct GeoWasm {
    inner: GeoStore,
}

// تغییر فیلد names به text برای سبک شدن خروجی و دریافت نام یکتا
#[derive(Serialize)]
pub struct DropdownItem {
    pub value: String,
    pub text: String,
}

#[wasm_bindgen]
impl GeoWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GeoWasm {
        GeoWasm {
            inner: GeoStore::load_embedded(),
        }
    }

    // ... (توابع addCountry، getCountry و غیره سر جای خودشان می‌مانند) ...

    // دریافت مستقیم زبان (locale) از جاوااسکریپت
    #[wasm_bindgen(js_name = getCountriesList)]
    pub fn get_countries_list(&self, locale: String) -> String {
        let mut list: Vec<DropdownItem> = self.inner.countries.values().map(|c| {
            let text = c.names.get(&locale)
                .or_else(|| c.names.get("en"))
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());
            
            DropdownItem { value: c.code.clone(), text }
        }).collect();
        
        // مرتب‌سازی هوشمند بر اساس زبان انتخابی
        list.sort_by(|a, b| a.text.cmp(&b.text)); 
        serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string())
    }

    #[wasm_bindgen(js_name = getProvincesList)]
    pub fn get_provinces_list(&self, country_code: String, locale: String) -> String {
        if let Some(country) = self.inner.get_country(&country_code) {
            let mut list: Vec<DropdownItem> = country.provinces.values().map(|p| {
                let text = p.names.get(&locale)
                    .or_else(|| p.names.get("en"))
                    .cloned()
                    .unwrap_or_else(|| "Unknown".to_string());

                DropdownItem { value: p.code.clone(), text }
            }).collect();
            
            list.sort_by(|a, b| a.text.cmp(&b.text));
            return serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
        }
        "[]".to_string()
    }

    #[wasm_bindgen(js_name = getCitiesList)]
    pub fn get_cities_list(&self, country_code: String, province_code: String, locale: String) -> String {
        if let Some(province) = self.inner.get_province(&country_code, &province_code) {
            let mut list: Vec<DropdownItem> = province.cities.values().map(|c| {
                let text = c.names.get(&locale)
                    .or_else(|| c.names.get("en"))
                    .cloned()
                    .unwrap_or_else(|| "Unknown".to_string());

                DropdownItem { value: c.id.to_string(), text }
            }).collect();
            
            list.sort_by(|a, b| a.text.cmp(&b.text));
            return serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
        }
        "[]".to_string()
    }
    
    #[wasm_bindgen]
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}