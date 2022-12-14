// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CompoundBoostExcelConfigData = Vec<CompoundBoostExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompoundBoostExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LFDGCKPFONC")]
    pub lfdgckpfonc: i64,
}

pub fn load() -> Result<CompoundBoostExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CompoundBoostExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
