// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type StateExcelConfigData = Vec<StateExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct StateExcelConfigDatum {
    #[serde(rename = "stateName")]
    pub state_name: String,

    #[serde(rename = "stateType")]
    pub state_type: Option<String>,

    #[serde(rename = "rank")]
    pub rank: Option<i64>,
}

pub fn load() -> Result<StateExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "StateExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
