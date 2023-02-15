// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TrialAvatarActivityDataExcelConfigData = Vec<TrialAvatarActivityDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerType {
    #[serde(rename = "TRIGGER_FINISH_CHALLENGE")]
    TriggerFinishChallenge,
}

pub fn load() -> Result<TrialAvatarActivityDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TrialAvatarActivityDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
