// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MatchingTextDataExcelConfigData = Vec<MatchingTextDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchingTextDataExcelConfigDatum {
    #[serde(rename = "matchId")]
    pub match_id: i64,

    #[serde(rename = "matchIconHash")]
    pub match_icon_hash: i64,

    #[serde(rename = "matchBtnName")]
    pub match_btn_name: String,

    #[serde(rename = "matchBtnTips")]
    pub match_btn_tips: String,

    #[serde(rename = "requirementDescTextMapHash")]
    pub requirement_desc_text_map_hash: i64,

    #[serde(rename = "matchLimitReasonDescTextMapHash")]
    pub match_limit_reason_desc_text_map_hash: i64,

    #[serde(rename = "limitWarningDescTextMapHash")]
    pub limit_warning_desc_text_map_hash: i64,

    #[serde(rename = "inviteGuestDescTextMapHash")]
    pub invite_guest_desc_text_map_hash: i64,

    #[serde(rename = "inviteHostDescTextMapHash")]
    pub invite_host_desc_text_map_hash: i64,

    #[serde(rename = "matchStartDesc")]
    pub match_start_desc: String,

    #[serde(rename = "matchTitleTextMapHash")]
    pub match_title_text_map_hash: i64,

    #[serde(rename = "matchSuccessDescTextMapHash")]
    pub match_success_desc_text_map_hash: i64,

    #[serde(rename = "LLNKLKEHEDC")]
    pub llnklkehedc: i64,
}

pub fn load() -> Result<MatchingTextDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MatchingTextDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
