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

pub type ActivityDeliveryDailyExcelConfigData = Vec<ActivityDeliveryDailyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityDeliveryDailyExcelConfigDatum {
    #[serde(rename = "dailyConfigId")]
    pub daily_config_id: i64,

    #[serde(rename = "deliveryQuestConfig")]
    pub delivery_quest_config: Vec<DeliveryQuestConfig>,

    #[serde(rename = "taskDesc")]
    pub task_desc: Vec<i64>,

    #[serde(rename = "dailyRewardId")]
    pub daily_reward_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DeliveryQuestConfig {
    #[serde(rename = "parentQuestId")]
    pub parent_quest_id: i64,

    #[serde(rename = "deliveryQuestId")]
    pub delivery_quest_id: i64,

    #[serde(rename = "startQuestId")]
    pub start_quest_id: i64,

    #[serde(rename = "talkQuestId")]
    pub talk_quest_id: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: Vec<i64>,

    #[serde(rename = "itemId")]
    pub item_id: i64,
}
