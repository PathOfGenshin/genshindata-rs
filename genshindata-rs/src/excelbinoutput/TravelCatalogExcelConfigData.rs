/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TravelCatalogExcelConfigData = Vec<TravelCatalogExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TravelCatalogExcelConfigDatum {
    pub id: i64,
    pub collect_id: i64,
    pub quest_id: i64,
    pub group_id: Option<i64>,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub icon: String,
}
