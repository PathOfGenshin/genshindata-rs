/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type InspirationSpurtLevelExcelConfigData = Vec<InspirationSpurtLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InspirationSpurtLevelExcelConfigDatum {
    pub level_id: i64,
    pub dungeon_id: i64,
    pub watcher_list: Vec<i64>,
    pub title_text_map_hash: i64,
    #[serde(rename = "LBEBKBHBIPO")]
    pub lbebkbhbipo: i64,
}
