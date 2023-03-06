// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TeamChainExcelConfigData = Vec<TeamChainExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamChainExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "IPDEINDKDBC")]
    pub ipdeindkdbc: Vec<i64>,

    #[serde(rename = "EFPAAONBHFE")]
    pub efpaaonbhfe: Vec<Vec<i64>>,

    #[serde(rename = "AKPEBHLFMEC")]
    pub akpebhlfmec: Vec<i64>,

    #[serde(rename = "HMHMAMDBAMM")]
    pub hmhmamdbamm: Vec<Vec<i64>>,
}

pub fn load() -> Result<TeamChainExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TeamChainExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
