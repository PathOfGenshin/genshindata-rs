// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DeshretCatalogDataData = Vec<DeshretCatalogDataDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeshretCatalogDataDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CHPHMBNCOHF")]
    pub chphmbncohf: i64,

    #[serde(rename = "OGDDGNOGHIO")]
    pub ogddgnoghio: i64,
}

pub fn load() -> Result<DeshretCatalogDataData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DeshretCatalogDataData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
