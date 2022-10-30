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

pub type ReunionWatcherExcelConfigData = Vec<ReunionWatcherExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReunionWatcherExcelConfigDatum {
    #[serde(rename = "watcherGroupId")]
    pub watcher_group_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "activateLevelRange")]
    pub activate_level_range: ActivateLevelRange,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "score")]
    pub score: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,

    #[serde(rename = "HJJOCDBLLJB")]
    pub hjjocdblljb: Option<i64>,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ActivateLevelRange {
    #[serde(rename = "10,30")]
    The1030,

    #[serde(rename = "10,61")]
    The1061,

    #[serde(rename = "28,61")]
    The2861,

    #[serde(rename = "30,40")]
    The3040,

    #[serde(rename = "31,61")]
    The3161,

    #[serde(rename = "40,50")]
    The4050,

    #[serde(rename = "50,61")]
    The5061,
}
