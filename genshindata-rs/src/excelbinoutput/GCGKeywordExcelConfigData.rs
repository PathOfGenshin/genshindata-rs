// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgKeywordExcelConfigData = Vec<GcgKeywordExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgKeywordExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}

pub fn load() -> Result<GcgKeywordExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGKeywordExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
