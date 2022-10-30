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

pub type MechanicusWatcherExcelConfigData = Vec<MechanicusWatcherExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MechanicusWatcherExcelConfigDatum {
    #[serde(rename = "rewardTokens")]
    pub reward_tokens: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

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
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}
