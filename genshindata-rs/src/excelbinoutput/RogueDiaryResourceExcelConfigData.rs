// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RogueDiaryResourceExcelConfigData = Vec<RogueDiaryResourceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueDiaryResourceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub rogue_diary_resource_excel_config_datum_type: String,

    #[serde(rename = "value")]
    pub value: i64,
}

pub fn load() -> Result<RogueDiaryResourceExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RogueDiaryResourceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
