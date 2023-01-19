// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LanV3CampStageExcelConfigData = Vec<LanV3CampStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV3CampStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "KHAKJEOMIAC")]
    pub khakjeomiac: Vec<i64>,

    #[serde(rename = "stageNameTextMapHash")]
    pub stage_name_text_map_hash: i64,

    #[serde(rename = "KKDEMBGOGMA")]
    pub kkdembgogma: Vec<i64>,
}

pub fn load() -> Result<LanV3CampStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LanV3CampStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
