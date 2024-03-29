/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RogueTokenExcelConfigData = Vec<RogueTokenExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RogueTokenExcelConfigDatum {
    pub id: i64,
    pub stage_id: i64,
    pub level: i64,
    pub coin_a_num: Vec<i64>,
    pub coin_b_num: Vec<i64>,
    pub coin_c_num: Vec<i64>,
    pub cell_type: Option<CellType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CellType {
    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_BOSS")]
    RogueMonsterDifficultyBoss,
    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_ELITE_EASY")]
    RogueMonsterDifficultyEliteEasy,
    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_ELITE_HARD")]
    RogueMonsterDifficultyEliteHard,
}
