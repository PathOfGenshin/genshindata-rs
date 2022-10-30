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

pub type GlobalWatcherConfigData = Vec<GlobalWatcherConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GlobalWatcherConfigDatum {
    #[serde(rename = "predicateConfigs")]
    pub predicate_configs: Vec<PredicateConfig>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PredicateConfig {
    #[serde(rename = "predicateType")]
    pub predicate_type: Option<PredicateType>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum PredicateType {
    #[serde(rename = "PREDICATE_CURRENT_AVATAR")]
    PredicateCurrentAvatar,

    #[serde(rename = "PREDICATE_QUEST_FINISH")]
    PredicateQuestFinish,
}
