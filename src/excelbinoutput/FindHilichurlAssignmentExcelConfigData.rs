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

pub type FindHilichurlAssignmentExcelConfigData = Vec<FindHilichurlAssignmentExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FindHilichurlAssignmentExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "index")]
    pub index: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "questID")]
    pub quest_id: i64,

    #[serde(rename = "keyWord")]
    pub key_word: String,

    #[serde(rename = "hintSubQuestId")]
    pub hint_sub_quest_id: i64,

    #[serde(rename = "assignmentType")]
    pub assignment_type: Option<String>,
}
