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

pub type TrialAvatarActivityDataExcelConfigData = Vec<TrialAvatarActivityDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TrialAvatarActivityDataExcelConfigDatum {
    #[serde(rename = "trialAvatarIndexId")]
    pub trial_avatar_index_id: i64,

    #[serde(rename = "trialAvatarId")]
    pub trial_avatar_id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "battleAvatarsList")]
    pub battle_avatars_list: String,

    #[serde(rename = "firstPassReward")]
    pub first_pass_reward: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "briefInfoTextMapHash")]
    pub brief_info_text_map_hash: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum TriggerType {
    #[serde(rename = "TRIGGER_FINISH_CHALLENGE")]
    TriggerFinishChallenge,
}
