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

pub type InstableSprayStageExcelConfigData = Vec<InstableSprayStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InstableSprayStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "BFNMFEFGECM")]
    pub bfnmfefgecm: i64,

    #[serde(rename = "DAFFNDPLMBM")]
    pub daffndplmbm: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "ECJLPFICKPL")]
    pub ecjlpfickpl: Vec<i64>,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
}
