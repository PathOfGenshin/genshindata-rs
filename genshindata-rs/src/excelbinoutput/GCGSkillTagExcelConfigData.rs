/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgSkillTagExcelConfigData = Vec<GcgSkillTagExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcgSkillTagExcelConfigDatum {
    #[serde(rename = "type")]
    pub gcg_skill_tag_excel_config_datum_type: String,
    pub name_text_map_hash: i64,
    #[serde(rename = "CLNHPPHELGO")]
    pub clnhpphelgo: Option<i64>,
}
