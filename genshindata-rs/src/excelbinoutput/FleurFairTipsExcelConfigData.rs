// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type FleurFairTipsExcelConfigData = Vec<FleurFairTipsExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FleurFairTipsExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}

pub fn load() -> Result<FleurFairTipsExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FleurFairTipsExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
