/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EffigyV4DifficultyExcelConfigData = Vec<EffigyV4DifficultyExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffigyV4DifficultyExcelConfigDatum {
    pub id: i64,
    pub challenge_id: i64,
    pub monster_difficulty: String,
    pub difficulty_desc_text_map_hash: i64,
    #[serde(rename = "FHHNOJNOMGA")]
    pub fhhnojnomga: i64,
    #[serde(rename = "JCIFLBEJDPP")]
    pub jciflbejdpp: i64,
    pub monster_level: i64,
    pub base_score: i64,
    pub score_ratio: f64,
}
