// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityMistTrialWatcherListDataExcelConfigData = Vec<ActivityMistTrialWatcherListDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
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

    #[serde(rename = "FCFBLCCFPPM")]
    pub fcfblccfppm: i64,

    #[serde(rename = "PDOPNECGJEE")]
    pub pdopnecgjee: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DungeonShowContentType {
    #[serde(rename = "MIST_TRIAL_SYNC_CHALLENGE_LEFT_TIME")]
    MistTrialSyncChallengeLeftTime,

    #[serde(rename = "MIST_TRIAL_SYNC_PROGRESS")]
    MistTrialSyncProgress,

    #[serde(rename = "MIST_TRIAL_SYNC_STAT_VALUE")]
    MistTrialSyncStatValue,
}

pub fn load() -> Result<ActivityMistTrialWatcherListDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityMistTrialWatcherListDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
