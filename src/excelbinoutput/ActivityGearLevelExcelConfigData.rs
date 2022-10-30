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

pub type ActivityGearLevelExcelConfigData = Vec<ActivityGearLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityGearLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "JKAJCMFKMGA")]
    pub jkajcmfkmga: Vec<i64>,

    #[serde(rename = "BEGKAFFEDAI")]
    pub begkaffedai: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "watcherID")]
    pub watcher_id: i64,

    #[serde(rename = "BOEOGCCNPKF")]
    pub boeogccnpkf: i64,

    #[serde(rename = "JLAFHFPPNIL")]
    pub jlafhfppnil: i64,
}
