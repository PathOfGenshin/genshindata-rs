// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CombatEndCleanExcelConfigData = Vec<CombatEndCleanExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatEndCleanExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LNAEMHBMFLE")]
    pub lnaemhbmfle: String,

    #[serde(rename = "NACHEDFCKLM")]
    pub nachedfcklm: Vec<Nachedfcklm>,

    #[serde(rename = "EEKEFPMEGHH")]
    pub eekefpmeghh: Vec<Eekefpmeghh>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Eekefpmeghh {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "LevelEntity_ClearLocalGadgets")]
    LevelEntityClearLocalGadgets,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Nachedfcklm {
    #[serde(rename = "Corruption")]
    Corruption,

    #[serde(rename = "None")]
    None,
}

pub fn load() -> Result<CombatEndCleanExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CombatEndCleanExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
