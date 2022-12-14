// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type FindHilichurlAssignmentExcelConfigData = Vec<FindHilichurlAssignmentExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindHilichurlAssignmentExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "index")]
    pub index: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "questID")]
    pub quest_id: i64,

    #[serde(rename = "keyWord")]
    pub key_word: String,

    #[serde(rename = "hintSubQuestId")]
    pub hint_sub_quest_id: i64,

    #[serde(rename = "assignmentType")]
    pub assignment_type: Option<String>,
}

pub fn load() -> Result<FindHilichurlAssignmentExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FindHilichurlAssignmentExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
