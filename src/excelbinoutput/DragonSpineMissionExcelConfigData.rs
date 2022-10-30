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

pub type DragonSpineMissionExcelConfigData = Vec<DragonSpineMissionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DragonSpineMissionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "finishExec")]
    pub finish_exec: Vec<FinishExec>,
}

#[derive(Serialize, Deserialize)]
pub struct FinishExec {
    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "type")]
    pub finish_exec_type: Option<String>,
}
