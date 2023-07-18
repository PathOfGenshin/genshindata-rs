/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RogueTokenExcelConfigData = Vec<RogueTokenExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RogueTokenExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "stageId")]
    pub stage_id: i64,
    #[serde(rename = "level")]
    pub level: i64,
    pub gcnajekckio: Vec<i64>,
    pub ddjhgnhbogm: Vec<i64>,
    pub alkdfedidld: Vec<i64>,
    pub plmjllbcipm: Option<Plmjllbcipm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Plmjllbcipm {
    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_BOSS")]
    RogueMonsterDifficultyBoss,
    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_ELITE_EASY")]
    RogueMonsterDifficultyEliteEasy,
    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_ELITE_HARD")]
    RogueMonsterDifficultyEliteHard,
}
