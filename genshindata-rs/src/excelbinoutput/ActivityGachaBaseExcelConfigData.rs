/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityGachaBaseExcelConfigData = Vec<ActivityGachaBaseExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityGachaBaseExcelConfigDatum {
    pub activity_id: i64,
    pub material_id: i64,
    pub elem_time: i64,
    pub task_content_id: i64,
    pub unlock_stage_id: i64,
    pub max_convert: i64,
    pub robot_limit: i64,
    pub robot_guar_num: i64,
    pub robot_hidden_first_guar_num: i64,
    pub robot_hidden_guar_num: i64,
    pub robot_hidden_guar_rate: i64,
    pub watcher_list: Vec<i64>,
    pub quest_list: Vec<i64>,
    pub reminder_id: i64,
    pub exchange_tips_cond: i64,
    pub free_mode_unlock_quest: i64,
}
