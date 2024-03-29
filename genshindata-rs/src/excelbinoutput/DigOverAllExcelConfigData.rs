/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DigOverAllExcelConfigData = Vec<DigOverAllExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DigOverAllExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "activityID")]
    pub activity_id: i64,
    pub duration: i64,
    #[serde(rename = "questUnlockID")]
    pub quest_unlock_id: i64,
    #[serde(rename = "questID")]
    pub quest_id: i64,
    #[serde(rename = "stageUnlockID")]
    pub stage_unlock_id: i64,
    pub reward_preview_id: i64,
    pub end_quest_show_cond: i64,
    pub end_quest_finish_cond: i64,
    #[serde(rename = "endQuestID")]
    pub end_quest_id: i64,
}
