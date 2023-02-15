// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DigStageDataExcelConfigData = Vec<DigStageDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DigStageDataExcelConfigDatum {
    #[serde(rename = "stageID")]
    pub stage_id: i64,

    #[serde(rename = "LLBGHCFIAGF")]
    pub llbghcfiagf: Option<i64>,

    #[serde(rename = "questID")]
    pub quest_id: Option<i64>,

    #[serde(rename = "FHKGFOCNMNI")]
    pub fhkgfocnmni: i64,

    #[serde(rename = "CJPKFALNMDO")]
    pub cjpkfalnmdo: i64,

    #[serde(rename = "OOBFELNJHLI")]
    pub oobfelnjhli: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: Option<i64>,

    #[serde(rename = "unlockTime")]
    pub unlock_time: i64,
}

pub fn load() -> Result<DigStageDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DigStageDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
