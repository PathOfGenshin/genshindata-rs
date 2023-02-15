// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HuntingMonsterExcelConfigData = Vec<HuntingMonsterExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HuntingMonsterExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "monsterId")]
    pub monster_id: i64,

    #[serde(rename = "affix")]
    pub affix: Vec<i64>,

    #[serde(rename = "abilityGroup")]
    pub ability_group: String,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "reviseLevelId")]
    pub revise_level_id: i64,

    #[serde(rename = "cityList")]
    pub city_list: Vec<i64>,

    #[serde(rename = "limitTime")]
    pub limit_time: i64,

    #[serde(rename = "searchPointNum")]
    pub search_point_num: Option<i64>,

    #[serde(rename = "HIOMBCNMNOB")]
    pub hiombcnmnob: Vec<Option<serde_json::Value>>,

    #[serde(rename = "clueTextIdList")]
    pub clue_text_id_list: Vec<i64>,

    #[serde(rename = "newsTextTextMapHash")]
    pub news_text_text_map_hash: i64,

    #[serde(rename = "traitTextTextMapHash")]
    pub trait_text_text_map_hash: i64,

    #[serde(rename = "GPJAHNBGMHH")]
    pub gpjahnbgmhh: i64,

    #[serde(rename = "GGBHAPFKOPD")]
    pub ggbhapfkopd: i64,

    #[serde(rename = "NBKNAHNFBDI")]
    pub nbknahnfbdi: i64,

    #[serde(rename = "NALBHOJIGGD")]
    pub nalbhojiggd: i64,

    #[serde(rename = "refreshCond")]
    pub refresh_cond: Vec<i64>,

    #[serde(rename = "createPosType")]
    pub create_pos_type: Option<String>,

    #[serde(rename = "EEONMDJOCAA")]
    pub eeonmdjocaa: Option<bool>,

    #[serde(rename = "KACBOILGOKA")]
    pub kacboilgoka: Option<i64>,

    #[serde(rename = "difficulty")]
    pub difficulty: Option<Difficulty>,

    #[serde(rename = "initialPose")]
    pub initial_pose: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "HUNTING_DIFFICULTY_HARD")]
    HuntingDifficultyHard,

    #[serde(rename = "HUNTING_DIFFICULTY_MEDIUM")]
    HuntingDifficultyMedium,
}

pub fn load() -> Result<HuntingMonsterExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HuntingMonsterExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
