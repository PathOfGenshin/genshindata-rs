/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WeaponPromoteExcelConfigData = Vec<WeaponPromoteExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponPromoteExcelConfigDatum {
    pub weapon_promote_id: i64,
    pub cost_items: Vec<CostItem>,
    pub add_props: Vec<AddProp>,
    pub unlock_max_level: i64,
    pub promote_level: Option<i64>,
    pub required_player_level: Option<i64>,
    pub coin_cost: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddProp {
    pub prop_type: PropType,
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_BASE_ATTACK")]
    FightPropBaseAttack,
    #[serde(rename = "FIGHT_PROP_CHARGE_EFFICIENCY")]
    FightPropChargeEfficiency,
    #[serde(rename = "FIGHT_PROP_CRITICAL")]
    FightPropCritical,
    #[serde(rename = "FIGHT_PROP_CRITICAL_HURT")]
    FightPropCriticalHurt,
    #[serde(rename = "FIGHT_PROP_ELEMENT_MASTERY")]
    FightPropElementMastery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub id: Option<i64>,
    pub count: Option<i64>,
}
