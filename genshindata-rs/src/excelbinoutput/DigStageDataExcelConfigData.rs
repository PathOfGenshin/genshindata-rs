/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DigStageDataExcelConfigData = Vec<DigStageDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DigStageDataExcelConfigDatum {
    #[serde(rename = "stageID")]
    pub stage_id: i64,
    #[serde(rename = "questUnlockID")]
    pub quest_unlock_id: Option<i64>,
    #[serde(rename = "questID")]
    pub quest_id: Option<i64>,
    #[serde(rename = "stageUnlockID")]
    pub stage_unlock_id: i64,
    pub quest_title_text_map_hash: i64,
    pub quest_desc_text_map_hash: i64,
    pub push_tips_id: Option<i64>,
    pub unlock_time: i64,
}
