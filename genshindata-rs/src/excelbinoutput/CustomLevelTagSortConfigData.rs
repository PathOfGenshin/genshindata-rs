// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CustomLevelTagSortConfigData = Vec<CustomLevelTagSortConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomLevelTagSortConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "BHFCCPHCMJI")]
    pub bhfccphcmji: String,

    #[serde(rename = "JLANMBFGBLG")]
    pub jlanmbfgblg: String,
}

pub fn load() -> Result<CustomLevelTagSortConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CustomLevelTagSortConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
