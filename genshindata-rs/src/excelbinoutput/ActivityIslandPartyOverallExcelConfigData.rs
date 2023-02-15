// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityIslandPartyOverallExcelConfigData = Vec<ActivityIslandPartyOverallExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityIslandPartyOverallExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "KMKBIPHECCD")]
    pub kmkbipheccd: i64,

    #[serde(rename = "CEDHIKDADHO")]
    pub cedhikdadho: i64,
}

pub fn load() -> Result<ActivityIslandPartyOverallExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityIslandPartyOverallExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
