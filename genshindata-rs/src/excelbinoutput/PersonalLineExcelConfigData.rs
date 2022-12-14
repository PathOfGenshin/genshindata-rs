// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type PersonalLineExcelConfigData = Vec<PersonalLineExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalLineExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "startQuestId")]
    pub start_quest_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: Vec<i64>,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "sortOrder")]
    pub sort_order: i64,
}

pub fn load() -> Result<PersonalLineExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "PersonalLineExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
