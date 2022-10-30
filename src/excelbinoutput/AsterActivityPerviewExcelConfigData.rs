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

pub type AsterActivityPerviewExcelConfigData = Vec<AsterActivityPerviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AsterActivityPerviewExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "unlockLevel")]
    pub unlock_level: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "specialRewardId")]
    pub special_reward_id: i64,

    #[serde(rename = "activityStayTime")]
    pub activity_stay_time: i64,

    #[serde(rename = "perfabChangeQuestId")]
    pub perfab_change_quest_id: i64,
}
