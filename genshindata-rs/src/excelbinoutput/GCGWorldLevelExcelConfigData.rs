/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgWorldLevelExcelConfigData = Vec<GcgWorldLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcgWorldLevelExcelConfigDatum {
    pub id: i64,
    pub npc_id: i64,
    pub level_id: i64,
    pub level_title_text_map_hash: i64,
    #[serde(rename = "EDJEGCFHFNA")]
    pub edjegcfhfna: i64,
    pub talk_id: i64,
    #[serde(rename = "ONOGDELOJEA")]
    pub onogdelojea: Option<i64>,
    pub unlock_cond: Option<UnlockCond>,
    #[serde(rename = "CBKKABADBBK")]
    pub cbkkabadbbk: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnlockCond {
    #[serde(rename = "GCG_LEVEL_UNLOCK_QUEST")]
    GcgLevelUnlockQuest,
}
