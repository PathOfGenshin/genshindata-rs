// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DeshretPoiCatalogDataData = Vec<DeshretPoiCatalogDataDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeshretPoiCatalogDataDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MOAGDEFEBCE")]
    pub moagdefebce: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "NGCDLFHHIBP")]
    pub ngcdlfhhibp: i64,

    #[serde(rename = "FFLANBJABHF")]
    pub fflanbjabhf: i64,

    #[serde(rename = "FPIFPJNBEEP")]
    pub fpifpjnbeep: i64,
}

pub fn load() -> Result<DeshretPoiCatalogDataData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DeshretPoiCatalogDataData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
