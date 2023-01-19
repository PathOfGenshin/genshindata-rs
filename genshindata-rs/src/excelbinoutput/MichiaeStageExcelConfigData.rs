// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MichiaeStageExcelConfigData = Vec<MichiaeStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MichiaeStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "DOPNMEEOEGD")]
    pub dopnmeeoegd: i64,

    #[serde(rename = "PIPGGDKFKPO")]
    pub pipggdkfkpo: i64,

    #[serde(rename = "tabNameTextMapHash")]
    pub tab_name_text_map_hash: i64,
}

pub fn load() -> Result<MichiaeStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MichiaeStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
