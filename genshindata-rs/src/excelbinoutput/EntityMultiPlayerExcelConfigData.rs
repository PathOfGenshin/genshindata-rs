/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EntityMultiPlayerExcelConfigData = Vec<EntityMultiPlayerExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityMultiPlayerExcelConfigDatum {
    pub id: i64,
    pub prop_per_vec: Vec<PropPerVec>,
    pub endure_num_vec: Vec<i64>,
    pub element_shield_per_vec: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropPerVec {
    pub prop_type: PropType,
    pub prop_value_vec: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_ATTACK_MP_PERCENT")]
    FightPropAttackMpPercent,
    #[serde(rename = "FIGHT_PROP_HP_MP_PERCENT")]
    FightPropHpMpPercent,
}
