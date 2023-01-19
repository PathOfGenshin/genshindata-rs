// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgWeekRefreshExcelConfigData = Vec<GcgWeekRefreshExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgWeekRefreshExcelConfigDatum {
    #[serde(rename = "PDPEMMHJDPI")]
    pub pdpemmhjdpi: i64,

    #[serde(rename = "KPLEGMKBBPN")]
    pub kplegmkbbpn: i64,

    #[serde(rename = "OEGEABHJJHB")]
    pub oegeabhjjhb: i64,

    #[serde(rename = "AJHPOGFNJEK")]
    pub ajhpogfnjek: Vec<Ajhpogfnjek>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ajhpogfnjek {
    #[serde(rename = "DKIDEHDPCAI")]
    pub dkidehdpcai: Vec<i64>,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,
}

pub fn load() -> Result<GcgWeekRefreshExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGWeekRefreshExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
