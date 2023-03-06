// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ExpeditionActivityPreviewExcelConfigData = Vec<ExpeditionActivityPreviewExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpeditionActivityPreviewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "KNNODHIEIOP")]
    pub knnodhieiop: i64,

    #[serde(rename = "KACGFAHELHN")]
    pub kacgfahelhn: i64,
}

pub fn load() -> Result<ExpeditionActivityPreviewExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ExpeditionActivityPreviewExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
