/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GravenInnocenceCampStageExcelConfigData = Vec<GravenInnocenceCampStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GravenInnocenceCampStageExcelConfigDatum {
    pub id: i64,
    pub open_day: i64,
    #[serde(rename = "EMOPNIGCGGE")]
    pub emopnigcgge: Vec<i64>,
    pub title_text_map_hash: i64,
}
