// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LevelTagResetExcelConfigData = Vec<LevelTagResetExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelTagResetExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "NJGJIHKPNEP")]
    pub njgjihkpnep: Vec<i64>,
}

pub fn load() -> Result<LevelTagResetExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LevelTagResetExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
