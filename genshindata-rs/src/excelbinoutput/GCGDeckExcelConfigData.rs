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
    pub nonckoebeai: i64,
    pub mbdjimpakld: Vec<i64>,
    pub allldmincmc: Vec<i64>,
    pub dnpddpmllfi: Vec<Option<serde_json::Value>>,
    pub hkialneoian: Vec<Option<serde_json::Value>>,
    pub alifjcclfek: Vec<Alifjcclfek>,
    pub fnjliaggkcd: Vec<bool>,
    pub cehmmhlidmo: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alifjcclfek {
    pub id: Option<i64>,
    #[serde(rename = "GLKMBJLELDO")]
    pub glkmbjleldo: Option<i64>,
}
