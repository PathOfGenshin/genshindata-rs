/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AkaFesRhythmLevelFishExcelConfigData = Vec<AkaFesRhythmLevelFishExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AkaFesRhythmLevelFishExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub ipbaiplecmh: f64,
    pub kobfdpeilgl: f64,
    pub ipecleaonkc: f64,
    pub kcpapoocnjd: f64,
    #[serde(rename = "type")]
    pub aka_fes_rhythm_level_fish_excel_config_datum_type: Option<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FORMAL")]
    Formal,
}
