// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReliquaryDecomposeExcelConfigData = Vec<ReliquaryDecomposeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReliquaryDecomposeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "IFODFNCCNCG")]
    pub ifodfnccncg: i64,

    #[serde(rename = "IHDHIFPGNJM")]
    pub ihdhifpgnjm: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "DLEKGLAIDKH")]
    pub dlekglaidkh: i64,

    #[serde(rename = "effectDescTextMapHash")]
    pub effect_desc_text_map_hash: i64,
}

pub fn load() -> Result<ReliquaryDecomposeExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ReliquaryDecomposeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
