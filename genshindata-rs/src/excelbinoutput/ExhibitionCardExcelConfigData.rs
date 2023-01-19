// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ExhibitionCardExcelConfigData = Vec<ExhibitionCardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExhibitionCardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "styleTextMapHash")]
    pub style_text_map_hash: i64,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "MLAMOLMECIA")]
    pub mlamolmecia: Option<Mlamolmecia>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mlamolmecia {
    #[serde(rename = "EXHIBITION_CARD_ORDER_GREATER_BETTER")]
    ExhibitionCardOrderGreaterBetter,

    #[serde(rename = "EXHIBITION_CARD_ORDER_LESS_BETTER")]
    ExhibitionCardOrderLessBetter,
}

pub fn load() -> Result<ExhibitionCardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ExhibitionCardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
