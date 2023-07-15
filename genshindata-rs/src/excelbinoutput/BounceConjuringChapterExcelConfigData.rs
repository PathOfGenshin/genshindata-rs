/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BounceConjuringChapterExcelConfigData = Vec<BounceConjuringChapterExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BounceConjuringChapterExcelConfigDatum {
    pub id: i64,
    pub chapter_id: i64,
    pub open_day: i64,
    pub gallery_id: i64,
    #[serde(rename = "KGDELEFMLLM")]
    pub kgdelefmllm: i64,
    pub watcher_id_list: Vec<i64>,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    #[serde(rename = "NINHLCLDOBL")]
    pub ninhlcldobl: Vec<i64>,
    #[serde(rename = "ODPLBLMAEDI")]
    pub odplblmaedi: Vec<i64>,
    #[serde(rename = "NDOMPGBBIKB")]
    pub ndompgbbikb: Vec<i64>,
    #[serde(rename = "KEPIHAHMFGI")]
    pub kepihahmfgi: Vec<Option<serde_json::Value>>,
    pub pos: Vec<f64>,
}
