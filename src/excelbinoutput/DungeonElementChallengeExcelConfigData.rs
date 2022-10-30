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

pub type DungeonElementChallengeExcelConfigData = Vec<DungeonElementChallengeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DungeonElementChallengeExcelConfigDatum {
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "trialAvatarId")]
    pub trial_avatar_id: Vec<i64>,

    #[serde(rename = "tutorialId")]
    pub tutorial_id: i64,
}
