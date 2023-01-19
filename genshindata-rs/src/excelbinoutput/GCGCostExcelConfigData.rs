// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgCostExcelConfigData = Vec<GcgCostExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgCostExcelConfigDatum {
    #[serde(rename = "type")]
    pub gcg_cost_excel_config_datum_type: String,

    #[serde(rename = "IEIJOKCHEOK")]
    pub ieijokcheok: i64,
}

pub fn load() -> Result<GcgCostExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGCostExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
