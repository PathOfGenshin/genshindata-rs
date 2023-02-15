// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RandTaskExcelConfigData = Vec<RandTaskExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RandTaskExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentType")]
    pub content_type: Option<String>,

    #[serde(rename = "reward")]
    pub reward: i64,

    #[serde(rename = "targetTextMapHash")]
    pub target_text_map_hash: i64,

    #[serde(rename = "enterDistance")]
    pub enter_distance: i64,

    #[serde(rename = "exitDistance")]
    pub exit_distance: i64,

    #[serde(rename = "needUI")]
    pub need_ui: Option<bool>,
}

pub fn load() -> Result<RandTaskExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RandTaskExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
