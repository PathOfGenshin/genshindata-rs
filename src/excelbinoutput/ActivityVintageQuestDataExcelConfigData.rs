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

pub type ActivityVintageQuestDataExcelConfigData = Vec<ActivityVintageQuestDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintageQuestDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "JKMIPDOFBCD")]
    pub jkmipdofbcd: i64,

    #[serde(rename = "chapterTitleTextMapHash")]
    pub chapter_title_text_map_hash: i64,

    #[serde(rename = "DGGMIFJMJBP")]
    pub dggmifjmjbp: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: Option<i64>,
}
