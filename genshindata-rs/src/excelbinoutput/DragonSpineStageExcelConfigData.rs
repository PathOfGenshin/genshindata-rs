// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type DragonSpineStageExcelConfigData = Vec<DragonSpineStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DragonSpineStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "coinIdList")]
    pub coin_id_list: Vec<i64>,

    #[serde(rename = "openday")]
    pub openday: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "missionIdList")]
    pub mission_id_list: Vec<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}

pub fn load() -> Result<DragonSpineStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DragonSpineStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
