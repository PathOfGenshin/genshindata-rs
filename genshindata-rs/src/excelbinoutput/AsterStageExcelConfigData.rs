// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type AsterStageExcelConfigData = Vec<AsterStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsterStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "stageNameTextMapHash")]
    pub stage_name_text_map_hash: i64,

    #[serde(rename = "openday")]
    pub openday: i64,

    #[serde(rename = "openQuestId")]
    pub open_quest_id: i64,

    #[serde(rename = "introTextMapHash")]
    pub intro_text_map_hash: i64,

    #[serde(rename = "storyTextMapHash")]
    pub story_text_map_hash: i64,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,

    #[serde(rename = "unlockScore")]
    pub unlock_score: Option<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}

pub fn load() -> Result<AsterStageExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "AsterStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}