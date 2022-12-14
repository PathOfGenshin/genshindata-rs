// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type DisplayItemExcelConfigData = Vec<DisplayItemExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayItemExcelConfigDatum {
    #[serde(rename = "typeDescTextMapHash")]
    pub type_desc_text_map_hash: i64,

    #[serde(rename = "rankLevel")]
    pub rank_level: i64,

    #[serde(rename = "displayType")]
    pub display_type: Option<DisplayType>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "itemType")]
    pub item_type: ItemType,

    #[serde(rename = "param")]
    pub param: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DisplayType {
    #[serde(rename = "BARTENDER_ITEM")]
    BartenderItem,

    #[serde(rename = "DEFAULT_ITEM")]
    DefaultItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "ITEM_DISPLAY")]
    ItemDisplay,
}

pub fn load() -> Result<DisplayItemExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DisplayItemExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
