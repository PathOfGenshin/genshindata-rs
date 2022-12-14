// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ChannellerSlabPreviewExcelConfigData = Vec<ChannellerSlabPreviewExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannellerSlabPreviewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "activityStayTime")]
    pub activity_stay_time: i64,

    #[serde(rename = "unlockQuestID")]
    pub unlock_quest_id: i64,

    #[serde(rename = "slabQuestID")]
    pub slab_quest_id: i64,

    #[serde(rename = "unlockPlayerLevel")]
    pub unlock_player_level: i64,

    #[serde(rename = "levelRewardGadgetID")]
    pub level_reward_gadget_id: i64,

    #[serde(rename = "isMaskAvatarReward")]
    pub is_mask_avatar_reward: Option<bool>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}

pub fn load() -> Result<ChannellerSlabPreviewExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ChannellerSlabPreviewExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
