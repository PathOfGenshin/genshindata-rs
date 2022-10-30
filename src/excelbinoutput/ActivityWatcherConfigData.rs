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

pub type ActivityWatcherConfigData = Vec<ActivityWatcherConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityWatcherConfigDatum {
    #[serde(rename = "RewardId")]
    pub reward_id: i64,

    #[serde(rename = "RewardPreview")]
    pub reward_preview: i64,

    #[serde(rename = "ActivitychallengetipsTextMapHash")]
    pub activitychallengetips_text_map_hash: i64,

    #[serde(rename = "ExtraActivitychallengetipsTextMapHash")]
    pub extra_activitychallengetips_text_map_hash: i64,

    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "TriggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "Progress")]
    pub progress: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "TriggerType")]
    pub trigger_type: String,

    #[serde(rename = "Param")]
    pub param: Vec<String>,
}
