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

pub type AsterLittleExcelConfigData = Vec<AsterLittleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AsterLittleExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "nextStageIdVec")]
    pub next_stage_id_vec: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "missionVec")]
    pub mission_vec: Vec<i64>,

    #[serde(rename = "watcherIdVec")]
    pub watcher_id_vec: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}
