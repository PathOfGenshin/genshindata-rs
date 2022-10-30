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

pub type MonsterMultiPlayerExcelConfigData = Vec<MonsterMultiPlayerExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MonsterMultiPlayerExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "PropPer")]
    pub prop_per: Vec<PropPer>,

    #[serde(rename = "EndureNum")]
    pub endure_num: Vec<i64>,

    #[serde(rename = "ElementShield")]
    pub element_shield: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct PropPer {
    #[serde(rename = "PropType")]
    pub prop_type: PropType,

    #[serde(rename = "PropValue")]
    pub prop_value: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_ATTACK_MP_PERCENT")]
    FightPropAttackMpPercent,

    #[serde(rename = "FIGHT_PROP_HP_MP_PERCENT")]
    FightPropHpMpPercent,
}
