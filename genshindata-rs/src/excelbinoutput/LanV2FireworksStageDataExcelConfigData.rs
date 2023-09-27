/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LanV2FireworksStageDataExcelConfigData = Vec<LanV2FireworksStageDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanV2FireworksStageDataExcelConfigDatum {
    pub stage_id: i64,
    pub open_day: i64,
    pub challenge_id_list: Vec<i64>,
    pub tab_name_text_map_hash: i64,
    pub quest_desc_text_map_hash: i64,
    pub guide_quest_id: Vec<i64>,
    pub guide_quest_reward_preview_id: i64,
}
