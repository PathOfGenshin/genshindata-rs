// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type AranaraCollectionExcelConfigData = Vec<AranaraCollectionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AranaraCollectionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CALPIGAKMIN")]
    pub calpigakmin: Calpigakmin,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Calpigakmin {
    #[serde(rename = "ARANARA_COLLECTION_TYPE_CATALOG_V1")]
    AranaraCollectionTypeCatalogV1,
}

pub fn load() -> Result<AranaraCollectionExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "AranaraCollectionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
