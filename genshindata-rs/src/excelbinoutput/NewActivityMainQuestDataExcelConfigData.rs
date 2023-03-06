// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type NewActivityMainQuestDataExcelConfigData = Vec<NewActivityMainQuestDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewActivityMainQuestDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "FFGHKFHNJKK")]
    pub ffghkfhnjkk: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,

    #[serde(rename = "IKBBOKCILNN")]
    pub ikbbokcilnn: i64,

    #[serde(rename = "chapterTitleTextMapHash")]
    pub chapter_title_text_map_hash: i64,

    #[serde(rename = "EHDEMGOKAML")]
    pub ehdemgokaml: i64,

    #[serde(rename = "JDFEPFFJNCO")]
    pub jdfepffjnco: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: Option<i64>,
}

pub fn load() -> Result<NewActivityMainQuestDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "NewActivityMainQuestDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
