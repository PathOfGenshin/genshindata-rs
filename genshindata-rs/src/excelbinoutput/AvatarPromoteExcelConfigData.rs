/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AvatarPromoteExcelConfigData = Vec<AvatarPromoteExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarPromoteExcelConfigDatum {
    pub avatar_promote_id: i64,
    pub promote_audio: String,
    pub cost_items: Vec<CostItem>,
    pub unlock_max_level: i64,
    pub add_props: Vec<AddProp>,
    pub promote_level: Option<i64>,
    pub scoin_cost: Option<i64>,
    pub required_player_level: Option<i64>,
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
    #[serde(rename = "FIGHT_PROP_ATTACK_PERCENT")]
    FightPropAttackPercent,
    #[serde(rename = "FIGHT_PROP_BASE_ATTACK")]
    FightPropBaseAttack,
    #[serde(rename = "FIGHT_PROP_BASE_DEFENSE")]
    FightPropBaseDefense,
    #[serde(rename = "FIGHT_PROP_BASE_HP")]
    FightPropBaseHp,
    #[serde(rename = "FIGHT_PROP_CHARGE_EFFICIENCY")]
    FightPropChargeEfficiency,
    #[serde(rename = "FIGHT_PROP_CRITICAL")]
    FightPropCritical,
    #[serde(rename = "FIGHT_PROP_CRITICAL_HURT")]
    FightPropCriticalHurt,
    #[serde(rename = "FIGHT_PROP_DEFENSE_PERCENT")]
    FightPropDefensePercent,
    #[serde(rename = "FIGHT_PROP_ELEC_ADD_HURT")]
    FightPropElecAddHurt,
    #[serde(rename = "FIGHT_PROP_ELEMENT_MASTERY")]
    FightPropElementMastery,
    #[serde(rename = "FIGHT_PROP_FIRE_ADD_HURT")]
    FightPropFireAddHurt,
    #[serde(rename = "FIGHT_PROP_GRASS_ADD_HURT")]
    FightPropGrassAddHurt,
    #[serde(rename = "FIGHT_PROP_HEAL_ADD")]
    FightPropHealAdd,
    #[serde(rename = "FIGHT_PROP_HP_PERCENT")]
    FightPropHpPercent,
    #[serde(rename = "FIGHT_PROP_ICE_ADD_HURT")]
    FightPropIceAddHurt,
    #[serde(rename = "FIGHT_PROP_PHYSICAL_ADD_HURT")]
    FightPropPhysicalAddHurt,
    #[serde(rename = "FIGHT_PROP_ROCK_ADD_HURT")]
    FightPropRockAddHurt,
    #[serde(rename = "FIGHT_PROP_WATER_ADD_HURT")]
    FightPropWaterAddHurt,
    #[serde(rename = "FIGHT_PROP_WIND_ADD_HURT")]
    FightPropWindAddHurt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub id: Option<i64>,
    pub count: Option<i64>,
}
