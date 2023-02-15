// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MichiaeWatcherExcelConfigData = Vec<MichiaeWatcherExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MichiaeWatcherExcelConfigDatum {
    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "ONGLICJMKEB")]
    pub onglicjmkeb: i64,
}

pub fn load() -> Result<MichiaeWatcherExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MichiaeWatcherExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
