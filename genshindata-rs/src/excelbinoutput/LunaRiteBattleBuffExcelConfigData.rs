// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LunaRiteBattleBuffExcelConfigData = Vec<LunaRiteBattleBuffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LunaRiteBattleBuffExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "EINIPCMDHJJ")]
    pub einipcmdhjj: String,

    #[serde(rename = "ACBBDPHEFNI")]
    pub acbbdphefni: i64,

    #[serde(rename = "KHBOJFAANDE")]
    pub khbojfaande: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,
}

pub fn load() -> Result<LunaRiteBattleBuffExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LunaRiteBattleBuffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
