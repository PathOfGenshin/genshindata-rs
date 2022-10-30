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

pub type ChannellerSlabLoopDungeonDifficultyExcelConfigData =
    Vec<ChannellerSlabLoopDungeonDifficultyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChannellerSlabLoopDungeonDifficultyExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "difficulty")]
    pub difficulty: String,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "baseScore")]
    pub base_score: i64,
}
