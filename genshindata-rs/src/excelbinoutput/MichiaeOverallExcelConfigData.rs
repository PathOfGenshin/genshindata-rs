/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MichiaeOverallExcelConfigData = Vec<MichiaeOverallExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MichiaeOverallExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    pub offering_id: i64,
    pub statue_group_id: i64,
    pub statue_config_id: i64,
    pub boss_watcher_list: Vec<i64>,
    pub statue_cd: i64,
    pub pray_effect_range: i64,
    pub dark_challenge_reward_range: i64,
    pub activity_scene_id: i64,
    pub boss_challenge_reward_range: i64,
    pub crystal_exp_material_id: i64,
    pub radar_open_level: i64,
    pub all_radar_open_level: i64,
}
