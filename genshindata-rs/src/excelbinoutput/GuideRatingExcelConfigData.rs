// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GuideRatingExcelConfigData = Vec<GuideRatingExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GuideRatingExcelConfigDatum {
    #[serde(rename = "channelID")]
    pub channel_id: i64,

    #[serde(rename = "isChinaServer")]
    pub is_china_server: Option<bool>,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "platform")]
    pub platform: Option<Platform>,

    #[serde(rename = "subChannelId")]
    pub sub_channel_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "PLATFORM_ANDROID")]
    PlatformAndroid,

    #[serde(rename = "PLATFORM_PC")]
    PlatformPc,
}

pub fn load() -> Result<GuideRatingExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GuideRatingExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
