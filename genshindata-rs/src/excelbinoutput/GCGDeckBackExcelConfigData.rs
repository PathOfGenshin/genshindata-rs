// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgDeckBackExcelConfigData = Vec<GcgDeckBackExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgDeckBackExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "MAEDEOPNNON")]
    pub maedeopnnon: String,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KPINCGJPICF")]
    pub kpincgjpicf: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "BDFNKMJCNBF")]
    pub bdfnkmjcnbf: i64,

    #[serde(rename = "id")]
    pub id: Option<i64>,
}

pub fn load() -> Result<GcgDeckBackExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGDeckBackExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
