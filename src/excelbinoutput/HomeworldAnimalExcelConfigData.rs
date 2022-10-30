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

pub type HomeworldAnimalExcelConfigData = Vec<HomeworldAnimalExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeworldAnimalExcelConfigDatum {
    #[serde(rename = "furnitureID")]
    pub furniture_id: i64,

    #[serde(rename = "monsterID")]
    pub monster_id: i64,

    #[serde(rename = "isRebirth")]
    pub is_rebirth: i64,

    #[serde(rename = "rebirthCD")]
    pub rebirth_cd: i64,
}
