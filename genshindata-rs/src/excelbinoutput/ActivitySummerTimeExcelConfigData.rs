/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySummerTimeExcelConfigData = Vec<ActivitySummerTimeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySummerTimeExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "unlockQuestID")]
    pub unlock_quest_id: i64,
    pub content_duration: i64,
    pub unlock_player_level: i64,
    #[serde(rename = "JKIEPACCCMM")]
    pub jkiepacccmm: i64,
    #[serde(rename = "BBIOJHADNAG")]
    pub bbiojhadnag: Vec<i64>,
    pub reward_preview: i64,
    pub quest_id_list: Vec<i64>,
}
