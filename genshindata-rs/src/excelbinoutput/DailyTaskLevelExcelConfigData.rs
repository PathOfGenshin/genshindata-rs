// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DailyTaskLevelExcelConfigData = Vec<DailyTaskLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyTaskLevelExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "minPlayerLevel")]
    pub min_player_level: i64,

    #[serde(rename = "maxPlayerLevel")]
    pub max_player_level: i64,

    #[serde(rename = "scoreDropId")]
    pub score_drop_id: i64,

    #[serde(rename = "scorePreviewRewardId")]
    pub score_preview_reward_id: i64,

    #[serde(rename = "groupReviseLevel")]
    pub group_revise_level: Option<i64>,
}

pub fn load() -> Result<DailyTaskLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DailyTaskLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
