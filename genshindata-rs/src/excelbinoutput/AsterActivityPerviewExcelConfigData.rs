/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AsterActivityPerviewExcelConfigData = Vec<AsterActivityPerviewExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsterActivityPerviewExcelConfigDatum {
    pub activity_id: i64,
    pub desc_text_map_hash: i64,
    pub unlock_level: i64,
    pub reward_preview_id: i64,
    pub watcher_list: Vec<i64>,
    pub special_reward_id: i64,
    pub activity_stay_time: i64,
    pub perfab_change_quest_id: i64,
}
