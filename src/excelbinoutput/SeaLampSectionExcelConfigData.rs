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

pub type SeaLampSectionExcelConfigData = Vec<SeaLampSectionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SeaLampSectionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sectionId")]
    pub section_id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "mainQuestId")]
    pub main_quest_id: i64,

    #[serde(rename = "miniQuestId")]
    pub mini_quest_id: Vec<i64>,

    #[serde(rename = "watcherIdVec")]
    pub watcher_id_vec: Vec<i64>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
}
