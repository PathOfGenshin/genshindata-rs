// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

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

    #[serde(rename = "PIIFGJOGCGI")]
    pub piifgjogcgi: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "challengeMissionWatcherList")]
    pub challenge_mission_watcher_list: Vec<i64>,

    #[serde(rename = "statisticsIdList")]
    pub statistics_id_list: Vec<i64>,

    #[serde(rename = "bgIconHashSuffix")]
    pub bg_icon_hash_suffix: i64,

    #[serde(rename = "bgIconHashPre")]
    pub bg_icon_hash_pre: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "dungeonFactorIdList")]
    pub dungeon_factor_id_list: Vec<i64>,

    #[serde(rename = "failTips")]
    pub fail_tips: Vec<String>,

    #[serde(rename = "ACHCCLDFJED")]
    pub achccldfjed: Vec<i64>,

    #[serde(rename = "MLNBPEMLEGO")]
    pub mlnbpemlego: Mlnbpemlego,

    #[serde(rename = "PCBHKDFDKEA")]
    pub pcbhkdfdkea: Vec<i64>,

    #[serde(rename = "HMHDECLCMEG")]
    pub hmhdeclcmeg: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mlnbpemlego {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "SGV_ABILITY_Mist_Level")]
    SgvAbilityMistLevel,
}

pub fn load() -> Result<ActivityMistTrialLevelDataExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityMistTrialLevelDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
