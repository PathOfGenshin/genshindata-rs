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

pub type StrengthenBasePointExcelConfigData = Vec<StrengthenBasePointExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct StrengthenBasePointExcelConfigDatum {
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "dungeonType")]
    pub dungeon_type: DungeonType,
}

#[derive(Serialize, Deserialize)]
pub enum DungeonType {
    #[serde(rename = "DUNGEON_TYPE_BOSS")]
    DungeonTypeBoss,

    #[serde(rename = "DUNGEON_TYPE_BREAK")]
    DungeonTypeBreak,

    #[serde(rename = "DUNGEON_TYPE_DAILY")]
    DungeonTypeDaily,

    #[serde(rename = "DUNGEON_TYPE_NORMAL")]
    DungeonTypeNormal,

    #[serde(rename = "DUNGEON_TYPE_TOWER")]
    DungeonTypeTower,
}
