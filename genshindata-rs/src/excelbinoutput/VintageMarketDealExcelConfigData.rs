// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketDealExcelConfigData = Vec<VintageMarketDealExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketDealExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "HPJGKNDALEP")]
    pub hpjgkndalep: Vec<Hpjgkndalep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hpjgkndalep {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "JPPFGFOCJDF")]
    pub jppfgfocjdf: Option<i64>,

    #[serde(rename = "LOAGDNDMMJC")]
    pub loagdndmmjc: Option<i64>,
}

pub fn load() -> Result<VintageMarketDealExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketDealExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
