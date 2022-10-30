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

pub type AsterMissionExcelConfigData = Vec<AsterMissionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AsterMissionExcelConfigDatum {
    #[serde(rename = "missionId")]
    pub mission_id: i64,

    #[serde(rename = "phase")]
    pub phase: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "transPointId")]
    pub trans_point_id: i64,

    #[serde(rename = "tracePoint")]
    pub trace_point: String,

    #[serde(rename = "perfabPathHashSuffix")]
    pub perfab_path_hash_suffix: Option<i64>,

    #[serde(rename = "perfabPathHashPre")]
    pub perfab_path_hash_pre: Option<i64>,
}
