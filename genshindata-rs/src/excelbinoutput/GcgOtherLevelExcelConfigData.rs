/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgOtherLevelExcelConfigData = Vec<GcgOtherLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcgOtherLevelExcelConfigDatum {
    pub level_id: i64,
    pub talk_id: Vec<i64>,
}
