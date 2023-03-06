// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityVintageDataExcelConfigData = Vec<ActivityVintageDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityVintageDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "JNADFPMBGDF")]
    pub jnadfpmbgdf: i64,

    #[serde(rename = "OOAPHHNEJJB")]
    pub ooaphhnejjb: i64,

    #[serde(rename = "CIHPNHENBGC")]
    pub cihpnhenbgc: Vec<i64>,

    #[serde(rename = "EKFBGEOEOLC")]
    pub ekfbgeoeolc: i64,

    #[serde(rename = "MJKKDEEKECF")]
    pub mjkkdeekecf: i64,

    #[serde(rename = "FNNEOFPKFDF")]
    pub fnneofpkfdf: i64,
}

pub fn load() -> Result<ActivityVintageDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityVintageDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
