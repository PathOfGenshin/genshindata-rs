// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type WeaponCodexExcelConfigData = Vec<WeaponCodexExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "weaponId")]
    pub weapon_id: i64,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,

    #[serde(rename = "showOnlyUnlocked")]
    pub show_only_unlocked: Option<bool>,
}

pub fn load() -> Result<WeaponCodexExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "WeaponCodexExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
