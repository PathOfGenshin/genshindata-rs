// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type AranaraCollectionExcelConfigData = Vec<AranaraCollectionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AranaraCollectionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "GANJPIKFHBK")]
    pub ganjpikfhbk: Ganjpikfhbk,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ganjpikfhbk {
    #[serde(rename = "ARANARA_COLLECTION_TYPE_CATALOG_V1")]
    AranaraCollectionTypeCatalogV1,
}

pub fn load() -> Result<AranaraCollectionExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "AranaraCollectionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
