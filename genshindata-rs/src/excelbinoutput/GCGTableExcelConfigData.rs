/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgTableExcelConfigData = Vec<GcgTableExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgTableExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub nkjjpndhgfi: Option<bool>,
    pub dbbppaeinhn: String,
    pub gcmppnkhdgj: String,
}
