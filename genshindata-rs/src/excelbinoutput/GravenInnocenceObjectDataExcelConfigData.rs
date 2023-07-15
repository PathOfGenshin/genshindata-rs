/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GravenInnocenceObjectDataExcelConfigData = Vec<GravenInnocenceObjectDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GravenInnocenceObjectDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub nmllicmafmn: Option<i64>,
    pub khgcobopioa: i64,
    pub dgnooheekkj: i64,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    pub lkcjbgcikoh: i64,
}
