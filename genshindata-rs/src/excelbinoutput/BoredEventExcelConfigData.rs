// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BoredEventExcelConfigData = Vec<BoredEventExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BoredEventExcelConfigDatum {
    #[serde(rename = "eventType")]
    pub event_type: String,

    #[serde(rename = "param")]
    pub param: i64,

    #[serde(rename = "isEnable")]
    pub is_enable: Option<bool>,

    #[serde(rename = "addBored")]
    pub add_bored: i64,

    #[serde(rename = "maxBored")]
    pub max_bored: i64,
}

pub fn load() -> Result<BoredEventExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BoredEventExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
