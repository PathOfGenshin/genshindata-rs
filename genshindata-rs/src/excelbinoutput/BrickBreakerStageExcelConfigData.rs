// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BrickBreakerStageExcelConfigData = Vec<BrickBreakerStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrickBreakerStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "HBJIHBHOCON")]
    pub hbjihbhocon: Vec<i64>,

    #[serde(rename = "IIGCOLKJKHN")]
    pub iigcolkjkhn: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "DPFGAJFHANI")]
    pub dpfgajfhani: Vec<Vec<i64>>,

    #[serde(rename = "DBEPLFEHALH")]
    pub dbeplfehalh: Vec<i64>,
}

pub fn load() -> Result<BrickBreakerStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BrickBreakerStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
