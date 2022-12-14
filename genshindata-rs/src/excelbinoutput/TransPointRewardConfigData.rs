// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TransPointRewardConfigData = Vec<TransPointRewardConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransPointRewardConfigDatum {
    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "pointId")]
    pub point_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "IMAIEIJPNHM")]
    pub imaieijpnhm: Vec<Option<serde_json::Value>>,

    #[serde(rename = "groupIdVec")]
    pub group_id_vec: Vec<i64>,
}

pub fn load() -> Result<TransPointRewardConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TransPointRewardConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
