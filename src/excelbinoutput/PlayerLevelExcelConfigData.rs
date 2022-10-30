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

pub type PlayerLevelExcelConfigData = Vec<PlayerLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PlayerLevelExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "exp")]
    pub exp: Option<i64>,

    #[serde(rename = "unlockDescTextMapHash")]
    pub unlock_desc_text_map_hash: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,

    #[serde(rename = "unlockWorldLevel")]
    pub unlock_world_level: Option<i64>,

    #[serde(rename = "expeditionLimitAdd")]
    pub expedition_limit_add: Option<i64>,
}
