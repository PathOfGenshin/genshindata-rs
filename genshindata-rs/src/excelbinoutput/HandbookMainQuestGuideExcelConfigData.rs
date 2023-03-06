// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HandbookMainQuestGuideExcelConfigData = Vec<HandbookMainQuestGuideExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HandbookMainQuestGuideExcelConfigDatum {
    #[serde(rename = "KPBOMHELLDM")]
    pub kpbomhelldm: i64,

    #[serde(rename = "OPOFKEAKHHO")]
    pub opofkeakhho: i64,

    #[serde(rename = "HGDEKCOMAKI")]
    pub hgdekcomaki: Vec<i64>,
}

pub fn load() -> Result<HandbookMainQuestGuideExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HandbookMainQuestGuideExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}