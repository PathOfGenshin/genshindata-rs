// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LanV2ProjectionSwitchButtonConfigData = Vec<LanV2ProjectionSwitchButtonConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV2ProjectionSwitchButtonConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "HPMLADPHEIK")]
    pub hpmladpheik: String,

    #[serde(rename = "iconPath")]
    pub icon_path: String,
}

pub fn load() -> Result<LanV2ProjectionSwitchButtonConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LanV2ProjectionSwitchButtonConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
