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

pub type ActivityRockBoardExploreStageExcelConfigData =
    Vec<ActivityRockBoardExploreStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityRockBoardExploreStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "EAJLPCOPPBP")]
    pub eajlpcoppbp: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "LFFAOCPCGIO")]
    pub lffaocpcgio: String,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "JKBJDCOJDLF")]
    pub jkbjdcojdlf: i64,

    #[serde(rename = "watcherID")]
    pub watcher_id: i64,
}
