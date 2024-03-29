/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TreasureMapExcelConfigData = Vec<TreasureMapExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasureMapExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    pub preview_reward_id: i64,
    pub npc_id: i64,
    pub npc_pos: Vec<f64>,
    pub treasure_days: i64,
    pub quest_id: i64,
    pub monster_probability: i64,
    pub reward_worktop_gadget_id: i64,
    pub token_material_id: i64,
    pub unit_token_drop_id: i64,
    pub mp_reward_id: i64,
    pub host_reward_limit: i64,
    pub guest_reward_limit: i64,
    pub mp_challenge_index: i64,
    pub bonus_challenge_index: i64,
    pub challenge_gadget_suite: i64,
    pub spot_revise_level_id: i64,
    pub detector_material_id: i64,
    pub guide_child_quest_id: i64,
    pub front_child_quest_id: Option<i64>,
}
