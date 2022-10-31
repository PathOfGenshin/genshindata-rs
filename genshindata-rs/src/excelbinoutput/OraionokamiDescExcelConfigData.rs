// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type OraionokamiDescExcelConfigData = Vec<OraionokamiDescExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OraionokamiDescExcelConfigDatum {
    #[serde(rename = "HNCJCMHHDMM")]
    pub hncjcmhhdmm: i64,

    #[serde(rename = "JANGAHGOCIC")]
    pub jangahgocic: i64,
}

pub fn load() -> Result<OraionokamiDescExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "OraionokamiDescExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}