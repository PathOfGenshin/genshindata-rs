/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TowerScheduleExcelConfigData = Vec<TowerScheduleExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerScheduleExcelConfigDatum {
    pub schedule_id: i64,
    pub entrance_floor_id: Vec<i64>,
    pub schedules: Vec<Schedule>,
    pub close_time: String,
    pub reward_group: i64,
    pub schedule_rewards: Vec<ScheduleReward>,
    pub monthly_level_config_id: i64,
    pub desc_text_map_hash: i64,
    pub buffname_text_map_hash: i64,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleReward {
    pub min_star_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub floor_list: Vec<i64>,
    pub open_time: String,
}
