// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type AvatarHeroEntityExcelConfigData = Vec<AvatarHeroEntityExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarHeroEntityExcelConfigDatum {
    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "DAHDIAPDAPB")]
    pub dahdiapdapb: i64,

    #[serde(rename = "gachaImageNameHash")]
    pub gacha_image_name_hash: i64,
}

pub fn load() -> Result<AvatarHeroEntityExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "AvatarHeroEntityExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
