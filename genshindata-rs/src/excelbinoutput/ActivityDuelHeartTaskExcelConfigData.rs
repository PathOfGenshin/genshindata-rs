// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityDuelHeartTaskExcelConfigData = Vec<ActivityDuelHeartTaskExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityDuelHeartTaskExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "BDHGNMNHEFG")]
    pub bdhgnmnhefg: i64,

    #[serde(rename = "CJPKFALNMDO")]
    pub cjpkfalnmdo: i64,

    #[serde(rename = "OOBFELNJHLI")]
    pub oobfelnjhli: i64,

    #[serde(rename = "EGADGIEKCAL")]
    pub egadgiekcal: i64,

    #[serde(rename = "NKEIDLGMAGJ")]
    pub nkeidlgmagj: i64,

    #[serde(rename = "DJFKFDDNEBE")]
    pub djfkfddnebe: Option<i64>,
}

pub fn load() -> Result<ActivityDuelHeartTaskExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityDuelHeartTaskExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
