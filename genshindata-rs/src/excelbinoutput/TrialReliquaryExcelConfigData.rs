// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TrialReliquaryExcelConfigData = Vec<TrialReliquaryExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrialReliquaryExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "ReliquaryId")]
    pub reliquary_id: i64,

    #[serde(rename = "Level")]
    pub level: i64,

    #[serde(rename = "MainPropId")]
    pub main_prop_id: i64,

    #[serde(rename = "AppendPropList")]
    pub append_prop_list: Vec<i64>,
}

pub fn load() -> Result<TrialReliquaryExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TrialReliquaryExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
