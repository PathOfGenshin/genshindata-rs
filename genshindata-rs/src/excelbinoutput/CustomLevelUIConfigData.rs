// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CustomLevelUiConfigData = Vec<CustomLevelUiConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomLevelUiConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: Option<i64>,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "BLAJAIABGGM")]
    pub blajaiabggm: Vec<i64>,
}

pub fn load() -> Result<CustomLevelUiConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CustomLevelUIConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
