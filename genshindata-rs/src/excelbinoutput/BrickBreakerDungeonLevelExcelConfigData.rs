// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BrickBreakerDungeonLevelExcelConfigData = Vec<BrickBreakerDungeonLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrickBreakerDungeonLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "LAPHKJFJAEO")]
    pub laphkjfjaeo: i64,

    #[serde(rename = "limitTime")]
    pub limit_time: i64,

    #[serde(rename = "KBONLOMIOFH")]
    pub kbonlomiofh: i64,

    #[serde(rename = "CFDKMHDHJHD")]
    pub cfdkmhdhjhd: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "GEEPALEMABO")]
    pub geepalemabo: i64,

    #[serde(rename = "JFLOONOAOBB")]
    pub jfloonoaobb: i64,

    #[serde(rename = "EFFIIFFKHHJ")]
    pub effiiffkhhj: Vec<i64>,

    #[serde(rename = "AEPACDKIALH")]
    pub aepacdkialh: Vec<String>,

    #[serde(rename = "EKKNKNFAIGI")]
    pub ekknknfaigi: Vec<i64>,
}

pub fn load() -> Result<BrickBreakerDungeonLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BrickBreakerDungeonLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
