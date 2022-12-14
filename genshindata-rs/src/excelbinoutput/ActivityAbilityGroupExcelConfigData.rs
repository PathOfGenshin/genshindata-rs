// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityAbilityGroupExcelConfigData = Vec<ActivityAbilityGroupExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityAbilityGroupExcelConfigDatum {
    #[serde(rename = "index")]
    pub index: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: Option<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "weaponId")]
    pub weapon_id: Option<i64>,
}

pub fn load() -> Result<ActivityAbilityGroupExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityAbilityGroupExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
