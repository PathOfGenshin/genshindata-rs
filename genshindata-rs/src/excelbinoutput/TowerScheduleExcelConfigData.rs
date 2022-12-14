// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TowerScheduleExcelConfigData = Vec<TowerScheduleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TowerScheduleExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "entranceFloorId")]
    pub entrance_floor_id: Vec<i64>,

    #[serde(rename = "schedules")]
    pub schedules: Vec<Schedule>,

    #[serde(rename = "closeTime")]
    pub close_time: String,

    #[serde(rename = "IPGAJMFELEC")]
    pub ipgajmfelec: i64,

    #[serde(rename = "scheduleRewards")]
    pub schedule_rewards: Vec<ScheduleReward>,

    #[serde(rename = "monthlyLevelConfigId")]
    pub monthly_level_config_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "buffnameTextMapHash")]
    pub buffname_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleReward {
    #[serde(rename = "minStarCount")]
    pub min_star_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "floorList")]
    pub floor_list: Vec<i64>,

    #[serde(rename = "openTime")]
    pub open_time: String,
}

pub fn load() -> Result<TowerScheduleExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TowerScheduleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
