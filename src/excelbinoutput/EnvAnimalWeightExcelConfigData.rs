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
use std::collections::HashMap;

pub type EnvAnimalWeightExcelConfigData = Vec<EnvAnimalWeightExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EnvAnimalWeightExcelConfigDatum {
    #[serde(rename = "envType")]
    pub env_type: String,

    #[serde(rename = "weightVec")]
    pub weight_vec: Vec<WeightVec>,
}

#[derive(Serialize, Deserialize)]
pub struct WeightVec {
    #[serde(rename = "animalId")]
    pub animal_id: Option<i64>,

    #[serde(rename = "entityType")]
    pub entity_type: Option<EntityType>,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,

    #[serde(rename = "aliveHourMap")]
    pub alive_hour_map: HashMap<String, i64>,
}

#[derive(Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "Gadget")]
    Gadget,

    #[serde(rename = "Monster")]
    Monster,
}
