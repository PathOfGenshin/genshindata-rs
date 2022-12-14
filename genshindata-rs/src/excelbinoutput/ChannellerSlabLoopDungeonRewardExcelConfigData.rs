// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ChannellerSlabLoopDungeonRewardExcelConfigData = Vec<ChannellerSlabLoopDungeonRewardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannellerSlabLoopDungeonRewardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "scoreGrade")]
    pub score_grade: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,
}

pub fn load() -> Result<ChannellerSlabLoopDungeonRewardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ChannellerSlabLoopDungeonRewardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
