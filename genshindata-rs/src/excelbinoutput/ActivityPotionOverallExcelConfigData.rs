/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityPotionOverallExcelConfigData = Vec<ActivityPotionOverallExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPotionOverallExcelConfigDatum {
    pub schedule_id: i64,
    pub score_param_a: f64,
    pub score_param_b: f64,
    pub score_param_c: f64,
}
