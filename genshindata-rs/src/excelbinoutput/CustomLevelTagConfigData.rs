// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type CustomLevelTagConfigData = Vec<CustomLevelTagConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomLevelTagConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "INLBFPHPKFJ")]
    pub inlbfphpkfj: i64,

    #[serde(rename = "sortId")]
    pub sort_id: i64,

    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
}

pub fn load() -> Result<CustomLevelTagConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "CustomLevelTagConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}