/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LuminanceStoneChallengeOverallExcelConfigData = Vec<LuminanceStoneChallengeOverallExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuminanceStoneChallengeOverallExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    pub final_stage_id: i64,
    pub final_gallery_id: i64,
    pub parent_quest_id_list: Vec<i64>,
    pub reward_preview_id: i64,
    pub gallery_rule_desc_text_map_hash: i64,
}
