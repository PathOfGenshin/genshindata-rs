// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DeshretPushTipsCatalogDataData = Vec<DeshretPushTipsCatalogDataDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeshretPushTipsCatalogDataDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "ACOMFLIELDA")]
    pub acomflielda: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "GEEEBDFHHKO")]
    pub geeebdfhhko: i64,

    #[serde(rename = "DPGOKCHIAEC")]
    pub dpgokchiaec: i64,
}

pub fn load() -> Result<DeshretPushTipsCatalogDataData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DeshretPushTipsCatalogDataData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
