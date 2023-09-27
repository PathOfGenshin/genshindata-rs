/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LuminanceStoneChallengeStageExcelConfigData = Vec<LuminanceStoneChallengeStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuminanceStoneChallengeStageExcelConfigDatum {
    pub stage_id: i64,
    pub day_index: i64,
    pub stage_title_text_map_hash: i64,
    pub stage_desc_text_map_hash: i64,
    pub stage_lock_title_text_map_hash: i64,
    pub stage_lock_desc_text_map_hash: i64,
    pub related_cond_id: Vec<i64>,
    pub watcher_list: Vec<i64>,
    pub quest_list: Vec<i64>,
    #[serde(rename = "BundleID")]
    pub bundle_id: i64,
    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
}
