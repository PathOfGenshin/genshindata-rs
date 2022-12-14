// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type SignInDayExcelConfigData = Vec<SignInDayExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInDayExcelConfigDatum {
    #[serde(rename = "ConfigId")]
    pub config_id: i64,

    #[serde(rename = "DayCount")]
    pub day_count: i64,

    #[serde(rename = "PeriodId")]
    pub period_id: i64,

    #[serde(rename = "RewardItemList")]
    pub reward_item_list: Vec<RewardItemList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardItemList {
    #[serde(rename = "ItemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "Count")]
    pub count: Option<i64>,

    #[serde(rename = "Quality")]
    pub quality: Option<i64>,
}

pub fn load() -> Result<SignInDayExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "SignInDayExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
