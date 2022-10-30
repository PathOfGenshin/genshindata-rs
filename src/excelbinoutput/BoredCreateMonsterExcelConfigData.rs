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

pub type BoredCreateMonsterExcelConfigData = Vec<BoredCreateMonsterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BoredCreateMonsterExcelConfigDatum {
    #[serde(rename = "playerLevel")]
    pub player_level: i64,

    #[serde(rename = "monsterConfigVec")]
    pub monster_config_vec: Vec<MonsterConfigVec>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterConfigVec {
    #[serde(rename = "weight")]
    pub weight: Option<i64>,

    #[serde(rename = "id")]
    pub id: Option<i64>,
}
