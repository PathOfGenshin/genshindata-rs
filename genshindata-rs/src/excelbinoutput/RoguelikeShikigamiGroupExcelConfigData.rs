// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RoguelikeShikigamiGroupExcelConfigData = Vec<RoguelikeShikigamiGroupExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoguelikeShikigamiGroupExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MOCCNJPNFCP")]
    pub moccnjpnfcp: Moccnjpnfcp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Moccnjpnfcp {
    #[serde(rename = "effectType")]
    pub effect_type: String,

    #[serde(rename = "PIOMJPCNDKA")]
    pub piomjpcndka: String,

    #[serde(rename = "AHIDGHIMNBK")]
    pub ahidghimnbk: String,
}

pub fn load() -> Result<RoguelikeShikigamiGroupExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RoguelikeShikigamiGroupExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
