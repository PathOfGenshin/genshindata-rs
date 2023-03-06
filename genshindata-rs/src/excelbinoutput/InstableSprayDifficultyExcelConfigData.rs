// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type InstableSprayDifficultyExcelConfigData = Vec<InstableSprayDifficultyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstableSprayDifficultyExcelConfigDatum {
    #[serde(rename = "HEFCEPGDJMI")]
    pub hefcepgdjmi: i64,

    #[serde(rename = "PKKOFEGEKFA")]
    pub pkkofegekfa: i64,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "GGINDMAEIJM")]
    pub ggindmaeijm: i64,
}

pub fn load() -> Result<InstableSprayDifficultyExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "InstableSprayDifficultyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
