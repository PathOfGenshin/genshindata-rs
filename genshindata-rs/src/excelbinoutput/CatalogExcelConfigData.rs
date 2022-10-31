// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type CatalogExcelConfigData = Vec<CatalogExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub catalog_excel_config_datum_type: String,

    #[serde(rename = "NEAGDHPBCGF")]
    pub neagdhpbcgf: Vec<Vec<i64>>,

    #[serde(rename = "JPBIPKDNPEO")]
    pub jpbipkdnpeo: Vec<Jpbipkdnpeo>,

    #[serde(rename = "GBLHPLPAKGE")]
    pub gblhplpakge: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jpbipkdnpeo {
    #[serde(rename = "DHBMIENBFMD")]
    pub dhbmienbfmd: Option<i64>,

    #[serde(rename = "LKOAEFLJCEC")]
    pub lkoaefljcec: Option<i64>,
}

pub fn load() -> Result<CatalogExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "CatalogExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}