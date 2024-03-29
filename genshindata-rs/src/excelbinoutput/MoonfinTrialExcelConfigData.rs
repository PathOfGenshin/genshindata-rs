/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MoonfinTrialExcelConfigData = Vec<MoonfinTrialExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoonfinTrialExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    pub level_id_list: Vec<i64>,
    pub activity_fish_id: i64,
    pub activity_fish_gain_limit: i64,
    pub activity_fish_pool_id: i64,
    pub activity_reward_preview_id: i64,
    pub push_tips_id: i64,
    pub main_quest_list: Vec<i64>,
    pub precondition_main_quest_ids: Vec<i64>,
}
