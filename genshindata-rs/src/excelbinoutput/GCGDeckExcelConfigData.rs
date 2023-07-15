/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgDeckExcelConfigData = Vec<GcgDeckExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgDeckExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub llkladaihgj: i64,
    pub lflghkkgnfm: Vec<i64>,
    pub pelmlacjnea: Vec<i64>,
    pub fnjojlbanda: Vec<Option<serde_json::Value>>,
    pub lakljnplhgm: Vec<Option<serde_json::Value>>,
    pub kgcajncahgj: Vec<Kgcajncahgj>,
    pub hneebphkoif: Vec<bool>,
    pub kghbjiilfie: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kgcajncahgj {
    pub id: Option<i64>,
    #[serde(rename = "IBBLAOCNBNG")]
    pub ibblaocnbng: Option<i64>,
}
