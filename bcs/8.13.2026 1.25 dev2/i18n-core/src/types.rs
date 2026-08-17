use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type Locale = String;
pub type IsoCode = String;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct City {
    pub id: u32,
    pub names: HashMap<Locale, String>,
    pub coordinates: Option<[f64; 2]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Province {
    pub code: IsoCode,
    pub names: HashMap<Locale, String>,
    pub cities: HashMap<u32, City>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Country {
    pub code: IsoCode,
    pub names: HashMap<Locale, String>,
    pub provinces: HashMap<IsoCode, Province>,
}