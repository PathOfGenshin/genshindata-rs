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

pub type QuestCodexExcelConfigData = Vec<QuestCodexExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct QuestCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "parentQuestId")]
    pub parent_quest_id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,
}
