/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CoopRewardExcelConfigData = Vec<CoopRewardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoopRewardExcelConfigDatum {
    pub id: i64,
    pub reward_cond: Vec<RewardCond>,
    pub chapter_id: i64,
    pub reward_id: i64,
    pub sort_id: i64,
    pub cond_tip_text_map_hash: i64,
    pub cond_tip_des_text_map_hash: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardCond {
    pub cond_type: CondType,
    pub args: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CondType {
    #[serde(rename = "COOP_COND_CHAPTER_END_FINISH_COUNT")]
    CoopCondChapterEndFinishCount,
}
