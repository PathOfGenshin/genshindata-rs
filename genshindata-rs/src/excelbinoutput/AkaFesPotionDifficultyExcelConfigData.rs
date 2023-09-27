/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AkaFesPotionDifficultyExcelConfigData = Vec<AkaFesPotionDifficultyExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AkaFesPotionDifficultyExcelConfigDatum {
    pub difficulty_level: i64,
    pub dungeon_level: i64,
    #[serde(rename = "MJEEGLFNFFH")]
    pub mjeeglfnffh: i64,
}
