/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgDeckStorageExcelConfigData = Vec<GcgDeckStorageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgDeckStorageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub akhagjacjlh: i64,
    #[serde(rename = "unlockCond")]
    pub unlock_cond: Option<UnlockCond>,
    pub cpbelmngnek: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnlockCond {
    #[serde(rename = "DUEL_LEVEL")]
    DuelLevel,
}
