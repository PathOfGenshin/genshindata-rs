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

pub type FleurFairPreviewExcelConfigData = Vec<FleurFairPreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FleurFairPreviewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "activityStayTime")]
    pub activity_stay_time: i64,

    #[serde(rename = "unlockQuestID")]
    pub unlock_quest_id: i64,

    #[serde(rename = "unlockPlayerLevel")]
    pub unlock_player_level: i64,

    #[serde(rename = "gameplayPreQuest")]
    pub gameplay_pre_quest: i64,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: i64,
}
