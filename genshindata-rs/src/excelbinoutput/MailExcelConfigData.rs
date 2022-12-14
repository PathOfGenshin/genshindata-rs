// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MailExcelConfigData = Vec<MailExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MailExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "expireDays")]
    pub expire_days: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "senderTextMapHash")]
    pub sender_text_map_hash: i64,

    #[serde(rename = "isStar")]
    pub is_star: Option<bool>,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,
}

pub fn load() -> Result<MailExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MailExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
