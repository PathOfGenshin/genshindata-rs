// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GroupLinksBundleRewardExcelConfigData = Vec<GroupLinksBundleRewardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupLinksBundleRewardExcelConfigDatum {
    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "rewardPreviewID")]
    pub reward_preview_id: Option<i64>,

    #[serde(rename = "dropID")]
    pub drop_id: Option<i64>,
}

pub fn load() -> Result<GroupLinksBundleRewardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GroupLinksBundleRewardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
