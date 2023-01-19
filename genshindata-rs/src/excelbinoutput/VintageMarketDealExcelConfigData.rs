// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type VintageMarketDealExcelConfigData = Vec<VintageMarketDealExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketDealExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "LCKLOBJNLCE")]
    pub lcklobjnlce: Vec<Lcklobjnlce>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lcklobjnlce {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "PEOEHBJNDBI")]
    pub peoehbjndbi: Option<i64>,

    #[serde(rename = "DFBGENIMMNF")]
    pub dfbgenimmnf: Option<i64>,
}

pub fn load() -> Result<VintageMarketDealExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketDealExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
