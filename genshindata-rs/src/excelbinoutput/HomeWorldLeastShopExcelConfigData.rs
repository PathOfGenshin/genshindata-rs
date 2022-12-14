// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HomeWorldLeastShopExcelConfigData = Vec<HomeWorldLeastShopExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldLeastShopExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "poolID")]
    pub pool_id: i64,

    #[serde(rename = "count")]
    pub count: i64,
}

pub fn load() -> Result<HomeWorldLeastShopExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeWorldLeastShopExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
