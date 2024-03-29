/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityGcgpveAffixExcelConfigData = Vec<ActivityGcgpveAffixExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityGcgpveAffixExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "type")]
    pub activity_gcgpve_affix_excel_config_datum_type: Type,
    pub param: i64,
    pub score: i64,
    pub desc_text_map_hash: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "GCG_AFFIX_CHALLENGE")]
    GcgAffixChallenge,
    #[serde(rename = "GCG_AFFIX_SKILL")]
    GcgAffixSkill,
}
