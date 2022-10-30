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

pub type ReliquaryLevelExcelConfigData = Vec<ReliquaryLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReliquaryLevelExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "addProps")]
    pub add_props: Vec<AddProp>,

    #[serde(rename = "rank")]
    pub rank: Option<i64>,

    #[serde(rename = "exp")]
    pub exp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AddProp {
    #[serde(rename = "propType")]
    pub prop_type: PropType,

    #[serde(rename = "value")]
    pub value: f64,
}

#[derive(Serialize, Deserialize)]
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_ATTACK")]
    FightPropAttack,

    #[serde(rename = "FIGHT_PROP_ATTACK_PERCENT")]
    FightPropAttackPercent,

    #[serde(rename = "FIGHT_PROP_CHARGE_EFFICIENCY")]
    FightPropChargeEfficiency,

    #[serde(rename = "FIGHT_PROP_CRITICAL")]
    FightPropCritical,

    #[serde(rename = "FIGHT_PROP_CRITICAL_HURT")]
    FightPropCriticalHurt,

    #[serde(rename = "FIGHT_PROP_DEFENSE")]
    FightPropDefense,

    #[serde(rename = "FIGHT_PROP_DEFENSE_PERCENT")]
    FightPropDefensePercent,

    #[serde(rename = "FIGHT_PROP_ELEC_ADD_HURT")]
    FightPropElecAddHurt,

    #[serde(rename = "FIGHT_PROP_ELEMENT_MASTERY")]
    FightPropElementMastery,

    #[serde(rename = "FIGHT_PROP_FIRE_ADD_HURT")]
    FightPropFireAddHurt,

    #[serde(rename = "FIGHT_PROP_FIRE_SUB_HURT")]
    FightPropFireSubHurt,

    #[serde(rename = "FIGHT_PROP_GRASS_ADD_HURT")]
    FightPropGrassAddHurt,

    #[serde(rename = "FIGHT_PROP_HEAL_ADD")]
    FightPropHealAdd,

    #[serde(rename = "FIGHT_PROP_HP")]
    FightPropHp,

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
