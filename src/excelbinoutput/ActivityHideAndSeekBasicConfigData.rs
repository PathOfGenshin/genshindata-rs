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

pub type ActivityHideAndSeekBasicConfigData = Vec<ActivityHideAndSeekBasicConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityHideAndSeekBasicConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,

    #[serde(rename = "rewardPreviewID")]
    pub reward_preview_id: i64,

    #[serde(rename = "oneTimeRewardPreviewID")]
    pub one_time_reward_preview_id: i64,

    #[serde(rename = "unlockQuestID")]
    pub unlock_quest_id: i64,

    #[serde(rename = "matchID")]
    pub match_id: i64,

    #[serde(rename = "draftID")]
    pub draft_id: i64,

    #[serde(rename = "scoreUnlockList")]
    pub score_unlock_list: Vec<i64>,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "skillList")]
    pub skill_list: Vec<i64>,

    #[serde(rename = "mapList")]
    pub map_list: Vec<i64>,

    #[serde(rename = "chanllengeList")]
    pub chanllenge_list: Vec<i64>,

    #[serde(rename = "scoreItemID")]
    pub score_item_id: i64,
}
