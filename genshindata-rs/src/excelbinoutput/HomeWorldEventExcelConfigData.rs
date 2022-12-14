// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HomeWorldEventExcelConfigData = Vec<HomeWorldEventExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldEventExcelConfigDatum {
    #[serde(rename = "NFLJELOCAKA")]
    pub nfljelocaka: i64,

    #[serde(rename = "OFHDECIHKIM")]
    pub ofhdecihkim: Ofhdecihkim,

    #[serde(rename = "avatarID")]
    pub avatar_id: i64,

    #[serde(rename = "LLKDBAPDNBJ")]
    pub llkdbapdnbj: Option<i64>,

    #[serde(rename = "rewardID")]
    pub reward_id: Option<i64>,

    #[serde(rename = "FNCOIDFIGPN")]
    pub fncoidfigpn: i64,

    #[serde(rename = "order")]
    pub order: Option<i64>,

    #[serde(rename = "MKOJPKOPLAM")]
    pub mkojpkoplam: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ofhdecihkim {
    #[serde(rename = "HOME_AVATAR_REWARD_EVENT")]
    HomeAvatarRewardEvent,

    #[serde(rename = "HOME_AVATAR_SUMMON_EVENT")]
    HomeAvatarSummonEvent,
}

pub fn load() -> Result<HomeWorldEventExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeWorldEventExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
