/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySkillExcelConfigData = Vec<ActivitySkillExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySkillExcelConfigDatum {
    pub id: i64,
    pub skill_target: Option<SkillTarget>,
    pub ability_name: String,
    pub global_value_key: String,
    pub energy_min: Option<i64>,
    pub energy_max: Option<i64>,
    pub skill_charge_phase: Vec<i64>,
    pub cd_time: i64,
    pub guide_time: Option<i64>,
    pub skill_icon: String,
    pub guide_key: Vec<String>,
    pub guide_open_state: Option<String>,
    pub unable_text_text_map_hash: i64,
    pub channel_text_text_map_hash: i64,
    pub interrupt_text_text_map_hash: i64,
    pub vfx_index: Option<i64>,
    pub skill_effect_hight_light: Option<bool>,
    #[serde(rename = "BIAEPMIGGBI")]
    pub biaepmiggbi: Option<bool>,
    #[serde(rename = "FHHJKILMEHH")]
    pub fhhjkilmehh: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SkillTarget {
    #[serde(rename = "AST_PLAY")]
    AstPlay,
    #[serde(rename = "AST_TEAM")]
    AstTeam,
    #[serde(rename = "AST_VEHICLE")]
    AstVehicle,
}
