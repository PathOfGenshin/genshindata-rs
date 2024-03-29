/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ElectroherculesBattleLevelExcelConfigData = Vec<ElectroherculesBattleLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectroherculesBattleLevelExcelConfigDatum {
    pub level_id: i64,
    #[serde(rename = "OCONPJOKMOB")]
    pub oconpjokmob: Oconpjokmob,
    pub group_id: i64,
    pub gallery_id: i64,
    pub watcher_id_list: Vec<i64>,
    pub level_type: LevelType,
    #[serde(rename = "CPAADIAIHLD")]
    pub cpaadiaihld: i64,
    #[serde(rename = "DKLIJMEMCOB")]
    pub dklijmemcob: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LevelType {
    #[serde(rename = "ELECTROHERCULES_BATTLE_LEVEL_TYPE_CHALLENGE")]
    ElectroherculesBattleLevelTypeChallenge,
    #[serde(rename = "ELECTROHERCULES_BATTLE_LEVEL_TYPE_GUIDE")]
    ElectroherculesBattleLevelTypeGuide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Oconpjokmob {
    #[serde(rename = "ELECTROHERCULES_BATTLE_DIFFICULTY_HARD")]
    ElectroherculesBattleDifficultyHard,
    #[serde(rename = "ELECTROHERCULES_BATTLE_DIFFICULTY_MASTER")]
    ElectroherculesBattleDifficultyMaster,
    #[serde(rename = "ELECTROHERCULES_BATTLE_DIFFICULTY_NORAML")]
    ElectroherculesBattleDifficultyNoraml,
}
