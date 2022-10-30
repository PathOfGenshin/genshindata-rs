// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type WidgetActiveExcelConfigData = Vec<WidgetActiveExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WidgetActiveExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "NFKEAHLDMOO")]
    pub nfkeahldmoo: Vec<Option<serde_json::Value>>,

    #[serde(rename = "OEIPJMOBLOB")]
    pub oeipjmoblob: String,

    #[serde(rename = "GLFIAPGPBCM")]
    pub glfiapgpbcm: Vec<i64>,

    #[serde(rename = "PDGMHLFNBFD")]
    pub pdgmhlfnbfd: Vec<i64>,

    #[serde(rename = "GKFJLONCCEO")]
    pub gkfjloncceo: Option<bool>,
}

pub fn load() -> Result<WidgetActiveExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "WidgetActiveExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
