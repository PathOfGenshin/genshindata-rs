// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgDeckBackExcelConfigData = Vec<GcgDeckBackExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgDeckBackExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "FMPJAAOEFNK")]
    pub fmpjaaoefnk: String,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KEMBEMMIJNB")]
    pub kembemmijnb: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "JFDMGHDNJCM")]
    pub jfdmghdnjcm: i64,

    #[serde(rename = "PPEDFCJEEAL")]
    pub ppedfcjeeal: i64,

    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "NJPPOMIEBLO")]
    pub njppomieblo: Option<bool>,
}

pub fn load() -> Result<GcgDeckBackExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGDeckBackExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
