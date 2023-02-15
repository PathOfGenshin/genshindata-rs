// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EffigyDifficultyExcelConfigData = Vec<EffigyDifficultyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EffigyDifficultyExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "challengeId")]
    pub challenge_id: i64,

    #[serde(rename = "baseScore")]
    pub base_score: i64,

    #[serde(rename = "monsterDifficulty")]
    pub monster_difficulty: MonsterDifficulty,

    #[serde(rename = "monsterLevel")]
    pub monster_level: i64,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "BGEJFFIOJAD")]
    pub bgejffiojad: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MonsterDifficulty {
    #[serde(rename = "EFFIGY_DIFFICULTY_EXPERT")]
    EffigyDifficultyExpert,

    #[serde(rename = "EFFIGY_DIFFICULTY_HARD")]
    EffigyDifficultyHard,

    #[serde(rename = "EFFIGY_DIFFICULTY_NORMAL")]
    EffigyDifficultyNormal,

    #[serde(rename = "EFFIGY_DIFFICULTY_PRIMER")]
    EffigyDifficultyPrimer,
}

pub fn load() -> Result<EffigyDifficultyExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EffigyDifficultyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
