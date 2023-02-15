// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BartenderAffixExcelConfigData = Vec<BartenderAffixExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BartenderAffixExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MLAMOLMECIA")]
    pub mlamolmecia: String,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "CHCNGHHDMAA")]
    pub chcnghhdmaa: i64,
}

pub fn load() -> Result<BartenderAffixExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BartenderAffixExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
