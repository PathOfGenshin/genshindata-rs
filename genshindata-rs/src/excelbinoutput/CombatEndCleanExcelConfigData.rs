/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CombatEndCleanExcelConfigData = Vec<CombatEndCleanExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CombatEndCleanExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub nebgdlbielo: String,
    pub mnecbflahmm: Vec<Mnecbflahmm>,
    pub kljcihahcjm: Vec<Kljcihahcjm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Kljcihahcjm {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "LevelEntity_ClearLocalGadgets")]
    LevelEntityClearLocalGadgets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mnecbflahmm {
    Corruption,
    None,
}
