/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DragonSpinePreviewExcelConfigData = Vec<DragonSpinePreviewExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragonSpinePreviewExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    pub desc_text_map_hash: i64,
    pub pre_quest_id: i64,
    pub unlock_level: i64,
    pub reward_preview_id: i64,
    pub content_duration: i64,
    pub quest_id: i64,
    pub quest_id_list: Vec<i64>,
}
