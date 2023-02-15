// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BrickBreakerQuestExcelConfigData = Vec<BrickBreakerQuestExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrickBreakerQuestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "stageID")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "KMJBNNBFBML")]
    pub kmjbnnbfbml: String,

    #[serde(rename = "levelID")]
    pub level_id: i64,

    #[serde(rename = "HONNJOMEIKP")]
    pub honnjomeikp: i64,

    #[serde(rename = "DJPGBDODPNI")]
    pub djpgbdodpni: i64,

    #[serde(rename = "CJPKFALNMDO")]
    pub cjpkfalnmdo: i64,

    #[serde(rename = "OOBFELNJHLI")]
    pub oobfelnjhli: i64,
}

pub fn load() -> Result<BrickBreakerQuestExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BrickBreakerQuestExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
