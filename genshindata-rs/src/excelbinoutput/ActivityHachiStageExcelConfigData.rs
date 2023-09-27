/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityHachiStageExcelConfigData = Vec<ActivityHachiStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHachiStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub stage_id: i64,
    pub quest_id: Vec<i64>,
    pub quest_title_text_map_hash: i64,
    pub quest_desc_text_map_hash: i64,
    pub stealth_title_text_map_hash: i64,
    pub battle_desc_text_map_hash: i64,
    pub stealth_watcher: i64,
    pub battle_watcher: i64,
    pub stealth_group: String,
    pub battle_group: String,
    pub push_tip: i64,
    pub open_day: i64,
    pub stealth_trigger_point_mark_pos: Vec<i64>,
    pub stealth_mark_pos: Vec<i64>,
    pub battle_mark_pos: Vec<i64>,
    pub final_quest_id: Vec<i64>,
    pub map_mark_load_quest: Vec<i64>,
    pub stealth_challenge_index: i64,
    pub battle_challenge_index: i64,
    pub stealth_radius: i64,
    pub battle_radius: i64,
}
