// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type PlayerLevelExcelConfigData = Vec<PlayerLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerLevelExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "exp")]
    pub exp: Option<i64>,

    #[serde(rename = "unlockDescTextMapHash")]
    pub unlock_desc_text_map_hash: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,

    #[serde(rename = "unlockWorldLevel")]
    pub unlock_world_level: Option<i64>,

    #[serde(rename = "expeditionLimitAdd")]
    pub expedition_limit_add: Option<i64>,
}

pub fn load() -> Result<PlayerLevelExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "PlayerLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
