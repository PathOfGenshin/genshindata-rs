// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityMistTrialLevelDataExcelConfigData = Vec<ActivityMistTrialLevelDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityMistTrialLevelDataExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "monsterPreviewIdList")]
    pub monster_preview_id_list: Vec<i64>,

    #[serde(rename = "ACCEILFOLCO")]
    pub acceilfolco: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "challengeMissionWatcherList")]
    pub challenge_mission_watcher_list: Vec<i64>,

    #[serde(rename = "statisticsIdList")]
    pub statistics_id_list: Vec<i64>,

    #[serde(rename = "HAANBOBJNIK")]
    pub haanbobjnik: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "dungeonFactorIdList")]
    pub dungeon_factor_id_list: Vec<i64>,

    #[serde(rename = "failTips")]
    pub fail_tips: Vec<String>,

    #[serde(rename = "NHDEEKOJELM")]
    pub nhdeekojelm: Vec<i64>,

    #[serde(rename = "DMNOGJKFFAG")]
    pub dmnogjkffag: Dmnogjkffag,

    #[serde(rename = "IBJLAKGMNMN")]
    pub ibjlakgmnmn: Vec<i64>,

    #[serde(rename = "OAENBOPDLCJ")]
    pub oaenbopdlcj: Vec<i64>,

    #[serde(rename = "bgIconHash")]
    pub bg_icon_hash: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Dmnogjkffag {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "SGV_ABILITY_Mist_Level")]
    SgvAbilityMistLevel,
}

pub fn load() -> Result<ActivityMistTrialLevelDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityMistTrialLevelDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
