// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CustomLevelGroupConfigData = Vec<CustomLevelGroupConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomLevelGroupConfigDatum {
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "dungeonList")]
    pub dungeon_list: Vec<i64>,

    #[serde(rename = "FBAGALOIMLI")]
    pub fbagaloimli: Vec<i64>,

    #[serde(rename = "NGNOLAMJCLC")]
    pub ngnolamjclc: i64,

    #[serde(rename = "LFJKJKCKBPH")]
    pub lfjkjkckbph: i64,

    #[serde(rename = "HECOKGJJBGK")]
    pub hecokgjjbgk: i64,

    #[serde(rename = "OKKFBCHLMID")]
    pub okkfbchlmid: i64,

    #[serde(rename = "PALLKIIDECL")]
    pub pallkiidecl: i64,

    #[serde(rename = "AOOPBMAOPDL")]
    pub aoopbmaopdl: i64,
}

pub fn load() -> Result<CustomLevelGroupConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CustomLevelGroupConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
