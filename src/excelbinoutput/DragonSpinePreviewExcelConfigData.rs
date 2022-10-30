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

pub type DragonSpinePreviewExcelConfigData = Vec<DragonSpinePreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DragonSpinePreviewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "unlockLevel")]
    pub unlock_level: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "contentDuration")]
    pub content_duration: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,
}
