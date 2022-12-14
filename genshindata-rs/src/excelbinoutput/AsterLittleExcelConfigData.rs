// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type AsterLittleExcelConfigData = Vec<AsterLittleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsterLittleExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "nextStageIdVec")]
    pub next_stage_id_vec: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "missionVec")]
    pub mission_vec: Vec<i64>,

    #[serde(rename = "watcherIdVec")]
    pub watcher_id_vec: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}

pub fn load() -> Result<AsterLittleExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "AsterLittleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
