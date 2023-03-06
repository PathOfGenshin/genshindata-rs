// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityPotionModeChoiceExcelConfigData = Vec<ActivityPotionModeChoiceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityPotionModeChoiceExcelConfigDatum {
    #[serde(rename = "LBLJFFFINOE")]
    pub lbljfffinoe: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "MKOBJIKDAKK")]
    pub mkobjikdakk: i64,
}

pub fn load() -> Result<ActivityPotionModeChoiceExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityPotionModeChoiceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
