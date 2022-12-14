// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ExpeditionChallengeExcelConfigData = Vec<ExpeditionChallengeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpeditionChallengeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "rewardChallengeIndex")]
    pub reward_challenge_index: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "challengeNameTextMapHash")]
    pub challenge_name_text_map_hash: i64,

    #[serde(rename = "challengeDescTextMapHash")]
    pub challenge_desc_text_map_hash: i64,

    #[serde(rename = "superElement")]
    pub super_element: String,

    #[serde(rename = "centerPosList")]
    pub center_pos_list: Vec<f64>,

    #[serde(rename = "centerRadius")]
    pub center_radius: i64,

    #[serde(rename = "unlockTime")]
    pub unlock_time: Option<i64>,
}

pub fn load() -> Result<ExpeditionChallengeExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ExpeditionChallengeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
