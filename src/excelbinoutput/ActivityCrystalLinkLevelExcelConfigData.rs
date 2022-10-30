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
use std::collections::HashMap;

pub type ActivityCrystalLinkLevelExcelConfigData = Vec<ActivityCrystalLinkLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityCrystalLinkLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "BNHKIIIIFMG")]
    pub bnhkiiiifmg: Vec<i64>,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "INLDGDKEOLM")]
    pub inldgdkeolm: Vec<i64>,

    #[serde(rename = "MFNALEPECFJ")]
    pub mfnalepecfj: Vec<i64>,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "MEHDCJLEHAC")]
    pub mehdcjlehac: Vec<HashMap<String, Vec<String>>>,

    #[serde(rename = "ALANAMPDJEE")]
    pub alanampdjee: Vec<HashMap<String, Vec<i64>>>,

    #[serde(rename = "scoreLevelList")]
    pub score_level_list: Vec<i64>,

    #[serde(rename = "KAEHEOCCFKP")]
    pub kaeheoccfkp: i64,

    #[serde(rename = "POKGEGPGFCM")]
    pub pokgegpgfcm: i64,
}
