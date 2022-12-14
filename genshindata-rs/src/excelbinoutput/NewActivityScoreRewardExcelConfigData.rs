// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type NewActivityScoreRewardExcelConfigData = Vec<NewActivityScoreRewardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewActivityScoreRewardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,

    #[serde(rename = "score")]
    pub score: i64,

    #[serde(rename = "activityScoreTipsTextMapHash")]
    pub activity_score_tips_text_map_hash: i64,

    #[serde(rename = "activityScoreExtraTipsTextMapHash")]
    pub activity_score_extra_tips_text_map_hash: i64,
}

pub fn load() -> Result<NewActivityScoreRewardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "NewActivityScoreRewardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
