/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LanV2FireworksSkillDataExcelConfigData = Vec<LanV2FireworksSkillDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanV2FireworksSkillDataExcelConfigDatum {
    pub skill_id: i64,
    pub skill_type: String,
    pub stamina_value_cost: i64,
    pub delta_fire_element_value: i64,
    pub lucky_prob: Option<i64>,
    pub factor_add_value_min: Option<i64>,
    pub factor_add_value_max: Option<i64>,
    pub effect_params: Vec<i64>,
    pub skill_title_text_map_hash: i64,
    pub skill_desc_text_map_hash: i64,
    pub desc_args: Vec<i64>,
    pub unlock_challenge_id: Option<i64>,
}
