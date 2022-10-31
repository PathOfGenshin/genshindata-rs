// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type FireworksFactorExcelConfigData = Vec<FireworksFactorExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FireworksFactorExcelConfigDatum {
    #[serde(rename = "IMGGODAFPAE")]
    pub imggodafpae: String,

    #[serde(rename = "PCANKJOPFIE")]
    pub pcankjopfie: i64,

    #[serde(rename = "MMILBCNCHJI")]
    pub mmilbcnchji: String,

    #[serde(rename = "JKBKMKPGJBB")]
    pub jkbkmkpgjbb: i64,

    #[serde(rename = "NLAJBKLHNPO")]
    pub nlajbklhnpo: i64,

    #[serde(rename = "JIFAHOIFLFK")]
    pub jifahoiflfk: i64,

    #[serde(rename = "FKNMKCFPIFI")]
    pub fknmkcfpifi: f64,
}

pub fn load() -> Result<FireworksFactorExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "FireworksFactorExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}