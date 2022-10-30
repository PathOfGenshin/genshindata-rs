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

pub type FleurFairChapterExcelConfigData = Vec<FleurFairChapterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FleurFairChapterExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "mainQuest")]
    pub main_quest: i64,

    #[serde(rename = "miniQuest")]
    pub mini_quest: Vec<i64>,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "preQuest")]
    pub pre_quest: Option<i64>,
}
