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

pub type EntityMultiPlayerExcelConfigData = Vec<EntityMultiPlayerExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EntityMultiPlayerExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "propPerVec")]
    pub prop_per_vec: Vec<PropPerVec>,

    #[serde(rename = "endureNumVec")]
    pub endure_num_vec: Vec<i64>,

    #[serde(rename = "elementShieldPerVec")]
    pub element_shield_per_vec: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct PropPerVec {
    #[serde(rename = "propType")]
    pub prop_type: PropType,

    #[serde(rename = "propValueVec")]
    pub prop_value_vec: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_ATTACK_MP_PERCENT")]
    FightPropAttackMpPercent,

    #[serde(rename = "FIGHT_PROP_HP_MP_PERCENT")]
    FightPropHpMpPercent,
}
