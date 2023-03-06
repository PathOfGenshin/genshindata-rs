// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LanV3CampStageExcelConfigData = Vec<LanV3CampStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV3CampStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "LKJEHDNDCNJ")]
    pub lkjehdndcnj: Vec<i64>,

    #[serde(rename = "stageNameTextMapHash")]
    pub stage_name_text_map_hash: i64,

    #[serde(rename = "CPHLJCHAGME")]
    pub cphljchagme: Vec<i64>,
}

pub fn load() -> Result<LanV3CampStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LanV3CampStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
