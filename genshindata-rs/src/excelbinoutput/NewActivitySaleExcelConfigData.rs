// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type NewActivitySaleExcelConfigData = Vec<NewActivitySaleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewActivitySaleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "saleType")]
    pub sale_type: String,

    #[serde(rename = "saleParam")]
    pub sale_param: Vec<String>,

    #[serde(rename = "bufftipsTextMapHash")]
    pub bufftips_text_map_hash: i64,
}

pub fn load() -> Result<NewActivitySaleExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "NewActivitySaleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
