// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type RogueCellWeightExcelConfigData = Vec<RogueCellWeightExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueCellWeightExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CGKKLOGJGFA")]
    pub cgkklogjgfa: i64,

    #[serde(rename = "LNFFFGFKHBG")]
    pub lnfffgfkhbg: i64,

    #[serde(rename = "CIJDFAEFFCN")]
    pub cijdfaeffcn: i64,
}

pub fn load() -> Result<RogueCellWeightExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "RogueCellWeightExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
