// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;
use std::collections::HashMap;

pub type FishStockExcelConfigData = Vec<FishStockExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FishStockExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub fish_stock_excel_config_datum_type: Type,

    #[serde(rename = "fishWeight")]
    pub fish_weight: HashMap<String, i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FISH_STOCK_TYPE_ANY")]
    FishStockTypeAny,

    #[serde(rename = "FISH_STOCK_TYPE_DAY")]
    FishStockTypeDay,

    #[serde(rename = "FISH_STOCK_TYPE_NIGHT")]
    FishStockTypeNight,
}

pub fn load() -> Result<FishStockExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FishStockExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
