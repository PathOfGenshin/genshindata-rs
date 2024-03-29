/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgWeekLevelExcelConfigData = Vec<GcgWeekLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgWeekLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "npcId")]
    pub npc_id: i64,
    pub pbpdendakpm: Pbpdendakpm,
    pub fgfdbccpaed: Vec<Fgfdbccpaed>,
    pub cglliibpnda: Vec<Option<serde_json::Value>>,
    #[serde(rename = "iconName")]
    pub icon_name: String,
    pub ooamkigiabh: Option<bool>,
    #[serde(rename = "openQuestId")]
    pub open_quest_id: Option<i64>,
    pub gnimpchikia: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fgfdbccpaed {
    #[serde(rename = "levelId")]
    pub level_id: Option<i64>,
    pub dccaejammfe: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Pbpdendakpm {
    #[serde(rename = "WEEK_NPC_CHARACTER")]
    WeekNpcCharacter,
    #[serde(rename = "WEEK_NPC_NORMAL")]
    WeekNpcNormal,
}
