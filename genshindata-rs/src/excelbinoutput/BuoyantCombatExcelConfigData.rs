// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BuoyantCombatExcelConfigData = Vec<BuoyantCombatExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuoyantCombatExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "KHAKJEOMIAC")]
    pub khakjeomiac: Vec<i64>,
}

pub fn load() -> Result<BuoyantCombatExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BuoyantCombatExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
