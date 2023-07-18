/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MonsterMultiPlayerExcelConfigData = Vec<MonsterMultiPlayerExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MonsterMultiPlayerExcelConfigDatum {
    pub id: i64,
    pub prop_per: Vec<PropPer>,
    pub endure_num: Vec<i64>,
    pub element_shield: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropPer {
    pub prop_type: PropType,
    pub prop_value: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_ATTACK_MP_PERCENT")]
    FightPropAttackMpPercent,
    #[serde(rename = "FIGHT_PROP_HP_MP_PERCENT")]
    FightPropHpMpPercent,
}
