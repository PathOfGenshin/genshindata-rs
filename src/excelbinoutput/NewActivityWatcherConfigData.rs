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

pub type NewActivityWatcherConfigData = Vec<NewActivityWatcherConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityWatcherConfigDatum {
    #[serde(rename = "rewardID")]
    pub reward_id: Option<i64>,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: Option<i64>,

    #[serde(rename = "activitychallengetipsTextMapHash")]
    pub activitychallengetips_text_map_hash: i64,

    #[serde(rename = "extraActivitychallengetipsTextMapHash")]
    pub extra_activitychallengetips_text_map_hash: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,

    #[serde(rename = "isAutoGrant")]
    pub is_auto_grant: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}
