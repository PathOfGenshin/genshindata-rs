// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MichiaeRadarExcelConfigData = Vec<MichiaeRadarExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MichiaeRadarExcelConfigDatum {
    #[serde(rename = "PDMHBBPJDPP")]
    pub pdmhbbpjdpp: String,

    #[serde(rename = "KMKHMPANHNM")]
    pub kmkhmpanhnm: Vec<i64>,
}

pub fn load() -> Result<MichiaeRadarExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MichiaeRadarExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
