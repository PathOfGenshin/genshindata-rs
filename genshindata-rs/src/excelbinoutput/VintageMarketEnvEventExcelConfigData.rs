// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketEnvEventExcelConfigData = Vec<VintageMarketEnvEventExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketEnvEventExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "duration")]
    pub duration: i64,

    #[serde(rename = "effectList")]
    pub effect_list: Vec<EffectList>,

    #[serde(rename = "EBMKMIGKGCB")]
    pub ebmkmigkgcb: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectList {
    #[serde(rename = "type")]
    pub effect_list_type: Option<String>,

    #[serde(rename = "param")]
    pub param: String,
}

pub fn load() -> Result<VintageMarketEnvEventExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketEnvEventExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
