// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LuminanceStoneChallengeOverallExcelConfigData = Vec<LuminanceStoneChallengeOverallExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LuminanceStoneChallengeOverallExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "DNFKECIFDJE")]
    pub dnfkecifdje: i64,

    #[serde(rename = "DEKEHGODHAK")]
    pub dekehgodhak: i64,

    #[serde(rename = "DMAEPPOBONC")]
    pub dmaeppobonc: Vec<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "AKEPBBFCFKN")]
    pub akepbbfcfkn: i64,
}

pub fn load() -> Result<LuminanceStoneChallengeOverallExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LuminanceStoneChallengeOverallExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
