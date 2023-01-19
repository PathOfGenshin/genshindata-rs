// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type EffigyChallengeV2TagExcelConfigData = Vec<EffigyChallengeV2TagExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EffigyChallengeV2TagExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LBIGHGOCIPN")]
    pub lbighgocipn: i64,

    #[serde(rename = "BFNEPMHODGA")]
    pub bfnepmhodga: Vec<Vec<f64>>,
}

pub fn load() -> Result<EffigyChallengeV2TagExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EffigyChallengeV2TagExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}