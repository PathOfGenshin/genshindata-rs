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

pub type WindFieldStageExcelConfigData = Vec<WindFieldStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WindFieldStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KKPNJFJBLNM")]
    pub kkpnjfjblnm: i64,

    #[serde(rename = "EFHNOEKDDEO")]
    pub efhnoekddeo: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,
}
