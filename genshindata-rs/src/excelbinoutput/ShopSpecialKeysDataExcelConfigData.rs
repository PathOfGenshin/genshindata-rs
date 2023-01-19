// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ShopSpecialKeysDataExcelConfigData = Vec<ShopSpecialKeysDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopSpecialKeysDataExcelConfigDatum {
    #[serde(rename = "goodsId")]
    pub goods_id: i64,

    #[serde(rename = "LGNNPAIJOIO")]
    pub lgnnpaijoio: i64,
}

pub fn load() -> Result<ShopSpecialKeysDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ShopSpecialKeysDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
