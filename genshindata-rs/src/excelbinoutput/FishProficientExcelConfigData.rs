// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type FishProficientExcelConfigData = Vec<FishProficientExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FishProficientExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "AGJLMKLCKDH")]
    pub agjlmklckdh: Vec<Agjlmklckdh>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agjlmklckdh {
    #[serde(rename = "JNBCKLCINDE")]
    pub jnbcklcinde: i64,

    #[serde(rename = "IMMIMMBFIJB")]
    pub immimmbfijb: f64,

    #[serde(rename = "POMPAGFJMOA")]
    pub pompagfjmoa: f64,
}

pub fn load() -> Result<FishProficientExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FishProficientExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
