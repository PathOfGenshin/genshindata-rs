// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ChargeBarStyleExcelConfigData = Vec<ChargeBarStyleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeBarStyleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "iconName")]
    pub icon_name: String,

    #[serde(rename = "GHCOOANOHAB")]
    pub ghcooanohab: Option<String>,
}

pub fn load() -> Result<ChargeBarStyleExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ChargeBarStyleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
