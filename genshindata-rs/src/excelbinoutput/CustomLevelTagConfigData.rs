// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CustomLevelTagConfigData = Vec<CustomLevelTagConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomLevelTagConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "NAEFINAPGFC")]
    pub naefinapgfc: i64,

    #[serde(rename = "sortId")]
    pub sort_id: i64,

    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
}

pub fn load() -> Result<CustomLevelTagConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CustomLevelTagConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
