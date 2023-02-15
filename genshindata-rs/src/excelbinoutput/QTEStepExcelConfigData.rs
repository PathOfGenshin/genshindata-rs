// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type QteStepExcelConfigData = Vec<QteStepExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct QteStepExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LADJCPNDHPC")]
    pub ladjcpndhpc: String,

    #[serde(rename = "HEDPOBFJJDP")]
    pub hedpobfjjdp: Option<i64>,
}

pub fn load() -> Result<QteStepExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "QTEStepExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
