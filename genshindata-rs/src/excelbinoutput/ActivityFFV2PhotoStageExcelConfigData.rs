/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityFfv2PhotoStageExcelConfigData = Vec<ActivityFfv2PhotoStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityFfv2PhotoStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "openDay")]
    pub open_day: i64,
    pub jlgnfpbdiok: Vec<i64>,
    pub ofdmlacooaj: i64,
    pub ehhilibakcd: String,
}
