// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LanV3TaskExcelConfigData = Vec<LanV3TaskExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV3TaskExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "condId")]
    pub cond_id: i64,
}

pub fn load() -> Result<LanV3TaskExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LanV3TaskExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
