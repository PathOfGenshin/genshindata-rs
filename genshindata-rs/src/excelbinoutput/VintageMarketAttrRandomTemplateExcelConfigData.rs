// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type VintageMarketAttrRandomTemplateExcelConfigData = Vec<VintageMarketAttrRandomTemplateExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketAttrRandomTemplateExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MGFNDJNOANP")]
    pub mgfndjnoanp: i64,

    #[serde(rename = "GIPHHNNJHII")]
    pub giphhnnjhii: Vec<i64>,
}

pub fn load() -> Result<VintageMarketAttrRandomTemplateExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketAttrRandomTemplateExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
