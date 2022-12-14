// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type EffigyChallengeV2DifficultyExcelConfigData = Vec<EffigyChallengeV2DifficultyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EffigyChallengeV2DifficultyExcelConfigDatum {
    #[serde(rename = "KALDOHCHJIH")]
    pub kaldohchjih: i64,

    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "reviseLevel")]
    pub revise_level: Option<i64>,
}

pub fn load() -> Result<EffigyChallengeV2DifficultyExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EffigyChallengeV2DifficultyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
