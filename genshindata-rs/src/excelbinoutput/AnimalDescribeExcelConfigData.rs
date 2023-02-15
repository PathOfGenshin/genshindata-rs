// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AnimalDescribeExcelConfigData = Vec<AnimalDescribeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalDescribeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,
}

pub fn load() -> Result<AnimalDescribeExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "AnimalDescribeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
