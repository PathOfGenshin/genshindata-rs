// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityPotionLevelExcelConfigData = Vec<ActivityPotionLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityPotionLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "HIAJJFFPOEF")]
    pub hiajjffpoef: i64,

    #[serde(rename = "KDJOHGJOJHB")]
    pub kdjohgjojhb: Vec<i64>,

    #[serde(rename = "NMKMCPCFIBA")]
    pub nmkmcpcfiba: Vec<i64>,

    #[serde(rename = "PCHMEFOBOLI")]
    pub pchmefoboli: Vec<i64>,

    #[serde(rename = "IMMGDJCLDAF")]
    pub immgdjcldaf: Vec<i64>,
}

pub fn load() -> Result<ActivityPotionLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityPotionLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
