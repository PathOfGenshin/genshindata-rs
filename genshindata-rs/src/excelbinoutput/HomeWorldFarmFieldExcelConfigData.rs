// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HomeWorldFarmFieldExcelConfigData = Vec<HomeWorldFarmFieldExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldFarmFieldExcelConfigDatum {
    #[serde(rename = "NIJMMICEOOL")]
    pub nijmmiceool: i64,

    #[serde(rename = "BEAAOJPJPFH")]
    pub beaaojpjpfh: String,

    #[serde(rename = "NLPIINAKJMI")]
    pub nlpiinakjmi: i64,

    #[serde(rename = "DAKEOEFHANE")]
    pub dakeoefhane: i64,

    #[serde(rename = "HKPABECEBNO")]
    pub hkpabecebno: Vec<i64>,
}

pub fn load() -> Result<HomeWorldFarmFieldExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeWorldFarmFieldExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
