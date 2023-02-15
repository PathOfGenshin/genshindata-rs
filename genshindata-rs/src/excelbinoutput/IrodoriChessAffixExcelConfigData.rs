// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriChessAffixExcelConfigData = Vec<IrodoriChessAffixExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriChessAffixExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}

pub fn load() -> Result<IrodoriChessAffixExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "IrodoriChessAffixExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
