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

pub type PlayerLevelLockExcelConfigData = Vec<PlayerLevelLockExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PlayerLevelLockExcelConfigDatum {
    #[serde(rename = "playerLevelUpperLimit")]
    pub player_level_upper_limit: i64,

    #[serde(rename = "unlockDescTextMapHash")]
    pub unlock_desc_text_map_hash: i64,

    #[serde(rename = "worldLevel")]
    pub world_level: Option<i64>,

    #[serde(rename = "unlockPlayerLevel")]
    pub unlock_player_level: Option<i64>,

    #[serde(rename = "unlockMainQuestId")]
    pub unlock_main_quest_id: Option<i64>,
}
