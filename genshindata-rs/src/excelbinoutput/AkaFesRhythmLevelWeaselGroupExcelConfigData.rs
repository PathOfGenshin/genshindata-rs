/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AkaFesRhythmLevelWeaselGroupExcelConfigData = Vec<AkaFesRhythmLevelWeaselGroupExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AkaFesRhythmLevelWeaselGroupExcelConfigDatum {
    #[serde(rename = "groupId")]
    pub group_id: i64,
    pub chbbfmgldff: i64,
    pub mbkdobfkili: f64,
    #[serde(rename = "type")]
    pub aka_fes_rhythm_level_weasel_group_excel_config_datum_type: Option<Type>,
    pub deocjkcdnoc: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FORMAL")]
    Formal,
}
