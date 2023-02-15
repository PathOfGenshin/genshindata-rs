// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketStoreExcelConfigData = Vec<VintageMarketStoreExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketStoreExcelConfigDatum {
    #[serde(rename = "DHICLENPCMF")]
    pub dhiclenpcmf: i64,

    #[serde(rename = "GHGBONFOENI")]
    pub ghgbonfoeni: i64,

    #[serde(rename = "DNALAPDMBKN")]
    pub dnalapdmbkn: i64,

    #[serde(rename = "ECELAIHBIJP")]
    pub ecelaihbijp: Vec<i64>,

    #[serde(rename = "FMNAEGPBHPH")]
    pub fmnaegpbhph: Vec<Fmnaegpbhph>,

    #[serde(rename = "FDBLBOEHJKM")]
    pub fdblboehjkm: i64,

    #[serde(rename = "FGANCCBLKIN")]
    pub fganccblkin: Vec<i64>,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "BDCLNEKOMIO")]
    pub bdclnekomio: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fmnaegpbhph {
    #[serde(rename = "defaultValue")]
    pub default_value: i64,

    #[serde(rename = "BNHPJBGHGOP")]
    pub bnhpjbghgop: i64,
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
