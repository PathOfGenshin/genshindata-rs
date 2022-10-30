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

pub type MpPlayWatcherConfigData = Vec<MpPlayWatcherConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MpPlayWatcherConfigDatum {
    #[serde(rename = "MpPlayId")]
    pub mp_play_id: i64,

    #[serde(rename = "challengeDescTextMapHash")]
    pub challenge_desc_text_map_hash: i64,

    #[serde(rename = "challengeTitleTextMapHash")]
    pub challenge_title_text_map_hash: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,

    #[serde(rename = "priority")]
    pub priority: Option<i64>,

    #[serde(rename = "isStore")]
    pub is_store: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}
