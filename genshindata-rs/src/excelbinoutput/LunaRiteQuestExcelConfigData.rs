// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LunaRiteQuestExcelConfigData = Vec<LunaRiteQuestExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LunaRiteQuestExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "chapterIcon")]
    pub chapter_icon: String,

    #[serde(rename = "MFGDPCGDBOL")]
    pub mfgdpcgdbol: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: Option<i64>,

    #[serde(rename = "AMKJPLIALEM")]
    pub amkjplialem: Option<i64>,

    #[serde(rename = "IBEBPNDNJHK")]
    pub ibebpndnjhk: Option<String>,
}

pub fn load() -> Result<LunaRiteQuestExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LunaRiteQuestExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
