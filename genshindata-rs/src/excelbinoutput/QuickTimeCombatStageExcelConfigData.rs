/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type QuickTimeCombatStageExcelConfigData = Vec<QuickTimeCombatStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickTimeCombatStageExcelConfigDatum {
    pub stage_id: i64,
    pub dungeon_id: i64,
    pub buff_id_list: Vec<i64>,
    pub trial_avatar_id_list: Vec<i64>,
    pub unlock_time: i64,
    #[serde(rename = "DPEECAALIDO")]
    pub dpeecaalido: i64,
    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
    pub watcher_list: Vec<i64>,
    pub level_title_text_map_hash: i64,
    pub level_description_text_map_hash: i64,
}