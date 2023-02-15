// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgShopTabExcelConfigData = Vec<GcgShopTabExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgShopTabExcelConfigDatum {
    #[serde(rename = "HJHGFEBNPLL")]
    pub hjhgfebnpll: i64,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "subTabList")]
    pub sub_tab_list: Vec<i64>,

    #[serde(rename = "shopType")]
    pub shop_type: String,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "order")]
    pub order: i64,
}

pub fn load() -> Result<GcgShopTabExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGShopTabExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
