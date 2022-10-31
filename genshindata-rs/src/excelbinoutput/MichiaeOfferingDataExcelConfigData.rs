// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MichiaeOfferingDataExcelConfigData = Vec<MichiaeOfferingDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MichiaeOfferingDataExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "OIPFAHBAMNA")]
    pub oipfahbamna: i64,

    #[serde(rename = "BNLGCJBHMOL")]
    pub bnlgcjbhmol: Vec<String>,

    #[serde(rename = "KEDMKFBECCK")]
    pub kedmkfbecck: i64,

    #[serde(rename = "IDDFPFFACDO")]
    pub iddfpffacdo: Vec<String>,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}

pub fn load() -> Result<MichiaeOfferingDataExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MichiaeOfferingDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}