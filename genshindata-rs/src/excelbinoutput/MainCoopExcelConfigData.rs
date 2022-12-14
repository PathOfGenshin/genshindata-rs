// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MainCoopExcelConfigData = Vec<MainCoopExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MainCoopExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "mainCoopCfg")]
    pub main_coop_cfg: String,

    #[serde(rename = "mainQuestSeries")]
    pub main_quest_series: i64,
}

pub fn load() -> Result<MainCoopExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MainCoopExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
