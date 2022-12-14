// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GroupTagExcelConfigData = Vec<GroupTagExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupTagExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LCENNMMJODO")]
    pub lcennmmjodo: String,
}

pub fn load() -> Result<GroupTagExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GroupTagExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
