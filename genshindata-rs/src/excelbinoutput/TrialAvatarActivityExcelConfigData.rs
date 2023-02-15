// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TrialAvatarActivityExcelConfigData = Vec<TrialAvatarActivityExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrialAvatarActivityExcelConfigDatum {
    #[serde(rename = "ScheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "AvatarIndexIdList")]
    pub avatar_index_id_list: Vec<i64>,

    #[serde(rename = "RewardIdList")]
    pub reward_id_list: Vec<i64>,
}

pub fn load() -> Result<TrialAvatarActivityExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TrialAvatarActivityExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
