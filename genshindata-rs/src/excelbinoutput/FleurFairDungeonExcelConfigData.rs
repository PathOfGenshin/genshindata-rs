/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FleurFairDungeonExcelConfigData = Vec<FleurFairDungeonExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleurFairDungeonExcelConfigDatum {
    pub id: i64,
    pub section_id: i64,
    pub open_day: i64,
    pub obtain_coin_limit: i64,
    pub activity_id: i64,
    pub watcher_id_list: Vec<i64>,
    pub dungeon_id: i64,
    pub mini_game_list: Vec<i64>,
    pub monster_list: Vec<i64>,
    pub energy_grade_list: Vec<i64>,
    pub initial_energy: i64,
    pub energy_limit: i64,
    pub success_reward_id: i64,
    pub failure_reward_id: i64,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
}
