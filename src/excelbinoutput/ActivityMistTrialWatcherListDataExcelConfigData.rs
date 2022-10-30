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

pub type ActivityMistTrialWatcherListDataExcelConfigData =
    Vec<ActivityMistTrialWatcherListDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityMistTrialWatcherListDataExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "challengeWatcherId")]
    pub challenge_watcher_id: i64,

    #[serde(rename = "dungeonShowContentType")]
    pub dungeon_show_content_type: DungeonShowContentType,

    #[serde(rename = "showParam")]
    pub show_param: Vec<String>,

    #[serde(rename = "progressFormatTextMapHash")]
    pub progress_format_text_map_hash: i64,

    #[serde(rename = "LJMEPBNDEIJ")]
    pub ljmepbndeij: i64,

    #[serde(rename = "ODDGENPKLPN")]
    pub oddgenpklpn: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum DungeonShowContentType {
    #[serde(rename = "MIST_TRIAL_SYNC_CHALLENGE_LEFT_TIME")]
    MistTrialSyncChallengeLeftTime,

    #[serde(rename = "MIST_TRIAL_SYNC_PROGRESS")]
    MistTrialSyncProgress,

    #[serde(rename = "MIST_TRIAL_SYNC_STAT_VALUE")]
    MistTrialSyncStatValue,
}
