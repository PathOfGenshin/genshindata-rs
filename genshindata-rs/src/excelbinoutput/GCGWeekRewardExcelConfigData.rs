/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type GcgWeekRewardExcelConfigData = Vec<GcgWeekRewardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgWeekRewardExcelConfigDatum {
    pub caiicjnnibm: i64,
    pub bnnbhoelfeg: Vec<HashMap<String, i64>>,
}
