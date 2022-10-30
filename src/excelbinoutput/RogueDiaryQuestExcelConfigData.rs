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

pub type RogueDiaryQuestExcelConfigData = Vec<RogueDiaryQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDiaryQuestExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<String>,

    #[serde(rename = "NOAHGNLNGIO")]
    pub noahgnlngio: Vec<i64>,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,
}
