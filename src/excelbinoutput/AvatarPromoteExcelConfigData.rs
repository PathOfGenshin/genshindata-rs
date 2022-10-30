// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type AvatarPromoteExcelConfigData = Vec<AvatarPromoteExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarPromoteExcelConfigDatum {
    #[serde(rename = "avatarPromoteId")]
    pub avatar_promote_id: i64,

    #[serde(rename = "promoteAudio")]
    pub promote_audio: String,

    #[serde(rename = "costItems")]
    pub cost_items: Vec<CostItem>,

    #[serde(rename = "unlockMaxLevel")]
    pub unlock_max_level: i64,

    #[serde(rename = "addProps")]
    pub add_props: Vec<AddProp>,

    #[serde(rename = "promoteLevel")]
    pub promote_level: Option<i64>,

    #[serde(rename = "scoinCost")]
    pub scoin_cost: Option<i64>,

    #[serde(rename = "requiredPlayerLevel")]
    pub required_player_level: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AddProp {
    #[serde(rename = "propType")]
    pub prop_type: PropType,

    #[serde(rename = "value")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct CostItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
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
