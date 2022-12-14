// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type AvatarCodexExcelConfigData = Vec<AvatarCodexExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarCodexExcelConfigDatum {
    #[serde(rename = "sortId")]
    pub sort_id: i64,

    #[serde(rename = "sortFactor")]
    pub sort_factor: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "beginTime")]
    pub begin_time: String,

    #[serde(rename = "HEDMBJMFEJP")]
    pub hedmbjmfejp: Option<bool>,
}

pub fn load() -> Result<AvatarCodexExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "AvatarCodexExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
