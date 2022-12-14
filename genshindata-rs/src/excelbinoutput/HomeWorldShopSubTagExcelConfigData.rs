// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HomeWorldShopSubTagExcelConfigData = Vec<HomeWorldShopSubTagExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldShopSubTagExcelConfigDatum {
    #[serde(rename = "subID")]
    pub sub_id: i64,

    #[serde(rename = "subTagTextMapHash")]
    pub sub_tag_text_map_hash: i64,

    #[serde(rename = "DBCDDJNNAJE")]
    pub dbcddjnnaje: Option<bool>,

    #[serde(rename = "CHHPFGNKKBL")]
    pub chhpfgnkkbl: Option<bool>,
}

pub fn load() -> Result<HomeWorldShopSubTagExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeWorldShopSubTagExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
