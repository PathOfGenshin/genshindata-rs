/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SeaLampSectionExcelConfigData = Vec<SeaLampSectionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeaLampSectionExcelConfigDatum {
    pub id: i64,
    pub section_id: i64,
    pub activity_id: i64,
    pub main_quest_id: i64,
    pub mini_quest_id: Vec<i64>,
    pub watcher_id_vec: Vec<i64>,
    pub desc_text_map_hash: i64,
    pub title_text_map_hash: i64,
    pub name_text_map_hash: i64,
}
