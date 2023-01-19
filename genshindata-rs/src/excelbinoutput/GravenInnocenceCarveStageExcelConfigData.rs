// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GravenInnocenceCarveStageExcelConfigData = Vec<GravenInnocenceCarveStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GravenInnocenceCarveStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "NEFLEHJGBGP")]
    pub neflehjgbgp: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,
}

pub fn load() -> Result<GravenInnocenceCarveStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GravenInnocenceCarveStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
