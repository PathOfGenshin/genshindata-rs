// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type QuestCatalogExcelConfigData = Vec<QuestCatalogExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestCatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MHALJGLGBPA")]
    pub mhaljglgbpa: Vec<i64>,

    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,

    #[serde(rename = "OLPBDHNGCIG")]
    pub olpbdhngcig: Vec<Option<serde_json::Value>>,

    #[serde(rename = "PHEFNODPEDM")]
    pub phefnodpedm: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "pic")]
    pub pic: String,
}

pub fn load() -> Result<QuestCatalogExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "QuestCatalogExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
