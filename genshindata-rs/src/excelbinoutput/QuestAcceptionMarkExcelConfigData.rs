/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type QuestAcceptionMarkExcelConfigData = Vec<QuestAcceptionMarkExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestAcceptionMarkExcelConfigDatum {
    pub config_id: i64,
    pub sub_quest_id: i64,
    pub npc_id: Option<i64>,
    pub is_daily_npc: Option<bool>,
    pub scene_id: i64,
    pub fallback_position: Vec<f64>,
    #[serde(rename = "blockMP")]
    pub block_mp: Option<bool>,
    pub desc_text_map_hash: i64,
    pub acceptable_time_tips_text_map_hash: i64,
}
