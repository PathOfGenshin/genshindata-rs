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

pub type ChannellerSlabDungeonExcelConfigData = Vec<ChannellerSlabDungeonExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChannellerSlabDungeonExcelConfigDatum {
    #[serde(rename = "stageID")]
    pub stage_id: i64,

    #[serde(rename = "dungeonID")]
    pub dungeon_id: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,

    #[serde(rename = "pointID")]
    pub point_id: i64,

    #[serde(rename = "pos")]
    pub pos: Vec<Option<serde_json::Value>>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}
