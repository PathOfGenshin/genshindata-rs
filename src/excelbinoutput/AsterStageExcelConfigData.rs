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

pub type AsterStageExcelConfigData = Vec<AsterStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AsterStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "stageNameTextMapHash")]
    pub stage_name_text_map_hash: i64,

    #[serde(rename = "openday")]
    pub openday: i64,

    #[serde(rename = "openQuestId")]
    pub open_quest_id: i64,

    #[serde(rename = "introTextMapHash")]
    pub intro_text_map_hash: i64,

    #[serde(rename = "storyTextMapHash")]
    pub story_text_map_hash: i64,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,

    #[serde(rename = "unlockScore")]
    pub unlock_score: Option<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}
