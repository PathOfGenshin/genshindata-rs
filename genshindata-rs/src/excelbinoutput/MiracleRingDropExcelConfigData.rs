// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MiracleRingDropExcelConfigData = Vec<MiracleRingDropExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MiracleRingDropExcelConfigDatum {
    #[serde(rename = "miracleTag")]
    pub miracle_tag: String,

    #[serde(rename = "dropId")]
    pub drop_id: Vec<i64>,
}

pub fn load() -> Result<MiracleRingDropExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MiracleRingDropExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}