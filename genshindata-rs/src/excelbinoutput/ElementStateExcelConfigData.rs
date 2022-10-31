// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ElementStateExcelConfigData = Vec<ElementStateExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementStateExcelConfigDatum {
    #[serde(rename = "elementType")]
    pub element_type: String,

    #[serde(rename = "elementIcon")]
    pub element_icon: String,

    #[serde(rename = "rank")]
    pub rank: i64,
}

pub fn load() -> Result<ElementStateExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ElementStateExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}