// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EchoShellStoryExcelConfigData = Vec<EchoShellStoryExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoShellStoryExcelConfigDatum {
    #[serde(rename = "storyTitleTextMapHash")]
    pub story_title_text_map_hash: i64,

    #[serde(rename = "id")]
    pub id: Option<i64>,
}

pub fn load() -> Result<EchoShellStoryExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EchoShellStoryExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
