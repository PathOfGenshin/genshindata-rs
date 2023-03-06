// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketStoreExcelConfigData = Vec<VintageMarketStoreExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketStoreExcelConfigDatum {
    #[serde(rename = "EOBGEBEDPNL")]
    pub eobgebedpnl: i64,

    #[serde(rename = "JAPPBBFEKJI")]
    pub jappbbfekji: i64,

    #[serde(rename = "FHGLFJFKBNC")]
    pub fhglfjfkbnc: i64,

    #[serde(rename = "PBNLBOEBALE")]
    pub pbnlboebale: Vec<i64>,

    #[serde(rename = "JMPOPBMGPOA")]
    pub jmpopbmgpoa: Vec<Jmpopbmgpoa>,

    #[serde(rename = "BIOOFEADFID")]
    pub bioofeadfid: i64,

    #[serde(rename = "FPIFFLMPFFE")]
    pub fpifflmpffe: Vec<i64>,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "FKOHNEFPECI")]
    pub fkohnefpeci: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jmpopbmgpoa {
    #[serde(rename = "defaultValue")]
    pub default_value: i64,

    #[serde(rename = "PIFOKBKPEEL")]
    pub pifokbkpeel: i64,
}

pub fn load() -> Result<VintageMarketStoreExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketStoreExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
