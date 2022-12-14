// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MatchPunishExcelConfigData = Vec<MatchPunishExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchPunishExcelConfigDatum {
    #[serde(rename = "times")]
    pub times: i64,

    #[serde(rename = "punishTime")]
    pub punish_time: i64,
}

pub fn load() -> Result<MatchPunishExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MatchPunishExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
