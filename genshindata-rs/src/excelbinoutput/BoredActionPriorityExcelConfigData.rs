// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BoredActionPriorityExcelConfigData = Vec<BoredActionPriorityExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BoredActionPriorityExcelConfigDatum {
    #[serde(rename = "actionType")]
    pub action_type: String,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,
}

pub fn load() -> Result<BoredActionPriorityExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BoredActionPriorityExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
