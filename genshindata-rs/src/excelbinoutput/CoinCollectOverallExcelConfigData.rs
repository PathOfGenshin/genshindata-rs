// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CoinCollectOverallExcelConfigData = Vec<CoinCollectOverallExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinCollectOverallExcelConfigDatum {
    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "IDHPMBJMOJL")]
    pub idhpmbjmojl: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "prepareTime")]
    pub prepare_time: i64,

    #[serde(rename = "EMHJJGKAGBG")]
    pub emhjjgkagbg: i64,

    #[serde(rename = "BHPLPFIBGJI")]
    pub bhplpfibgji: i64,

    #[serde(rename = "DNKCBBMMBEL")]
    pub dnkcbbmmbel: i64,
}

pub fn load() -> Result<CoinCollectOverallExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CoinCollectOverallExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
