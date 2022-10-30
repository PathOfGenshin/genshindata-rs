// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type GravenInnocenceCampLevelExcelConfigData = Vec<GravenInnocenceCampLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GravenInnocenceCampLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "PGGOKANNJLJ")]
    pub pggokannjlj: i64,

    #[serde(rename = "LHEGIFNCNDA")]
    pub lhegifncnda: i64,

    #[serde(rename = "LCMILPPPBCD")]
    pub lcmilpppbcd: i64,
}

pub fn load() -> Result<GravenInnocenceCampLevelExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "GravenInnocenceCampLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
