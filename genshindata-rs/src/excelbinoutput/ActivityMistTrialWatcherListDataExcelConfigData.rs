/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityMistTrialWatcherListDataExcelConfigData = Vec<ActivityMistTrialWatcherListDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMistTrialWatcherListDataExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub schedule_id: i64,
    pub challenge_watcher_id: i64,
    pub dungeon_show_content_type: DungeonShowContentType,
    pub show_param: Vec<String>,
    pub progress_format_text_map_hash: i64,
    pub hint_format_text_map_hash: i64,
    pub is_need_show_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DungeonShowContentType {
    #[serde(rename = "MIST_TRIAL_SYNC_CHALLENGE_LEFT_TIME")]
    MistTrialSyncChallengeLeftTime,
    #[serde(rename = "MIST_TRIAL_SYNC_PROGRESS")]
    MistTrialSyncProgress,
    #[serde(rename = "MIST_TRIAL_SYNC_STAT_VALUE")]
    MistTrialSyncStatValue,
}
