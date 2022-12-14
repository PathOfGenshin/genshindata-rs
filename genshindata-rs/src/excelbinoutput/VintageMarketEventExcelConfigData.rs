// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type VintageMarketEventExcelConfigData = Vec<VintageMarketEventExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketEventExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,

    #[serde(rename = "FMNEJFCAFGD")]
    pub fmnejfcafgd: String,

    #[serde(rename = "OFHDECIHKIM")]
    pub ofhdecihkim: Ofhdecihkim,

    #[serde(rename = "NPGJMIJDNOL")]
    pub npgjmijdnol: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ofhdecihkim {
    #[serde(rename = "VINTAGE_MARKET_EVENT_ENV")]
    VintageMarketEventEnv,

    #[serde(rename = "VINTAGE_MARKET_EVENT_NPC")]
    VintageMarketEventNpc,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerType {
    #[serde(rename = "VINTAGE_MARKET_EVENT_TRIGGER_ROUND")]
    VintageMarketEventTriggerRound,
}

pub fn load() -> Result<VintageMarketEventExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketEventExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
