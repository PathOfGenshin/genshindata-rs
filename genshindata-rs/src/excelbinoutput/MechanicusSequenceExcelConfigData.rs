// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MechanicusSequenceExcelConfigData = Vec<MechanicusSequenceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MechanicusSequenceExcelConfigDatum {
    #[serde(rename = "sequenceID")]
    pub sequence_id: i64,

    #[serde(rename = "mechanicusID")]
    pub mechanicus_id: i64,

    #[serde(rename = "openLevel")]
    pub open_level: i64,

    #[serde(rename = "openGearList")]
    pub open_gear_list: Vec<i64>,

    #[serde(rename = "gearLevelLimite")]
    pub gear_level_limite: i64,

    #[serde(rename = "gearMoneyLimite")]
    pub gear_money_limite: i64,

    #[serde(rename = "openMapList")]
    pub open_map_list: Vec<i64>,

    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "rewardPreviewID")]
    pub reward_preview_id: i64,
}

pub fn load() -> Result<MechanicusSequenceExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MechanicusSequenceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
