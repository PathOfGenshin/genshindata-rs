/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgPickGroupExcelConfigData = Vec<GcgPickGroupExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgPickGroupExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub pijicpmebip: i64,
    pub pjhdfglglfj: Vec<i64>,
}