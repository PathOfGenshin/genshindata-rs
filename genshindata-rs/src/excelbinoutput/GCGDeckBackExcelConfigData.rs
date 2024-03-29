/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgDeckBackExcelConfigData = Vec<GcgDeckBackExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgDeckBackExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,
    pub gjfmnfedncf: String,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    pub akhagjacjlh: i64,
    #[serde(rename = "order")]
    pub order: i64,
    pub cmcfanfnijn: f64,
    pub mmopidckngb: f64,
    #[serde(rename = "id")]
    pub id: Option<i64>,
    pub ocfhdpageli: Option<bool>,
}
