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

pub type AsterMidDifficultyExcelConfigData = Vec<AsterMidDifficultyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AsterMidDifficultyExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "worldLevelVec")]
    pub world_level_vec: Vec<i64>,

    #[serde(rename = "dropId")]
    pub drop_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "resin")]
    pub resin: i64,

    #[serde(rename = "recommendLevel")]
    pub recommend_level: i64,

    #[serde(rename = "monsterLevel")]
    pub monster_level: i64,
}
