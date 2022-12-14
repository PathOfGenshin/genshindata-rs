// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityArenaChallengeLevelInfoExcelConfigData = Vec<ActivityArenaChallengeLevelInfoExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityArenaChallengeLevelInfoExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "monsterPreviewId")]
    pub monster_preview_id: i64,

    #[serde(rename = "monsterConfig")]
    pub monster_config: String,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "LCAIHLNENBE")]
    pub lcaihlnenbe: String,

    #[serde(rename = "AGFPHLDFNJA")]
    pub agfphldfnja: i64,

    #[serde(rename = "LFPCAOEKMKJ")]
    pub lfpcaoekmkj: String,

    #[serde(rename = "LONKFHFGGMP")]
    pub lonkfhfggmp: i64,

    #[serde(rename = "POKAHPLIAGM")]
    pub pokahpliagm: String,

    #[serde(rename = "levelDetailDescTextMapHash")]
    pub level_detail_desc_text_map_hash: i64,

    #[serde(rename = "challengeIdList")]
    pub challenge_id_list: Vec<i64>,
}

pub fn load() -> Result<ActivityArenaChallengeLevelInfoExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityArenaChallengeLevelInfoExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
