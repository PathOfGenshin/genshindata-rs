/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EffigyV4LimitingConditionExcelConfigData = Vec<EffigyV4LimitingConditionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffigyV4LimitingConditionExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "GJCIHNLAAKJ")]
    pub gjcihnlaakj: i64,
    pub condition_type: ConditionType,
    #[serde(rename = "LNDAOBEJIGM")]
    pub lndaobejigm: String,
    #[serde(rename = "PEIBFAMCLOG")]
    pub peibfamclog: i64,
    pub exclusive_id: Option<i64>,
    pub score: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConditionType {
    #[serde(rename = "EFFIGY_V4_CONDITION_LEVEL_CONFIG")]
    EffigyV4ConditionLevelConfig,
    #[serde(rename = "EFFIGY_V4_CONDITION_TIME_LIMIT")]
    EffigyV4ConditionTimeLimit,
}
