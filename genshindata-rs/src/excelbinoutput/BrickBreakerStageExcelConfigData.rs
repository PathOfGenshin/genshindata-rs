// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BrickBreakerStageExcelConfigData = Vec<BrickBreakerStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrickBreakerStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "BHKNKPLLHAF")]
    pub bhknkpllhaf: Vec<i64>,

    #[serde(rename = "OMHMDNEFHKP")]
    pub omhmdnefhkp: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "IBNFFCGMMGK")]
    pub ibnffcgmmgk: Vec<Vec<i64>>,

    #[serde(rename = "FPOIKAHKCKH")]
    pub fpoikahkckh: Vec<i64>,
}

pub fn load() -> Result<BrickBreakerStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BrickBreakerStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
