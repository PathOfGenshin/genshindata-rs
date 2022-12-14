// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityPotionModeChoiceExcelConfigData = Vec<ActivityPotionModeChoiceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityPotionModeChoiceExcelConfigDatum {
    #[serde(rename = "NLAKFEJOJLE")]
    pub nlakfejojle: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "OMHMDNEFHKP")]
    pub omhmdnefhkp: i64,
}

pub fn load() -> Result<ActivityPotionModeChoiceExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityPotionModeChoiceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
