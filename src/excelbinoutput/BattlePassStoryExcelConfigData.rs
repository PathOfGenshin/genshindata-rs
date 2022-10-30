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

pub type BattlePassStoryExcelConfigData = Vec<BattlePassStoryExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BattlePassStoryExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "storyUnlockLevel")]
    pub story_unlock_level: Vec<i64>,

    #[serde(rename = "storyId")]
    pub story_id: Vec<i64>,

    #[serde(rename = "storyTitle")]
    pub story_title: Vec<i64>,
}
