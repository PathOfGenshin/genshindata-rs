/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcSuiteExcelConfigData = Vec<UgcSuiteExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UgcSuiteExcelConfigDatum {
    pub okgphbgajln: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "typeID")]
    pub type_id: i64,
    pub fgdknngbjha: Vec<i64>,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    #[serde(rename = "iconHash")]
    pub icon_hash: f64,
    #[serde(rename = "cost")]
    pub cost: i64,
    #[serde(rename = "rotateType")]
    pub rotate_type: String,
    pub eaakebfejel: String,
    pub mjdhlgoegkh: i64,
    pub iobaeplgdkf: bool,
    pub afkmehnnmki: bool,
}
