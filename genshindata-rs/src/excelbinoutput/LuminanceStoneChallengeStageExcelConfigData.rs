/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LuminanceStoneChallengeStageExcelConfigData = Vec<LuminanceStoneChallengeStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LuminanceStoneChallengeStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,
    #[serde(rename = "dayIndex")]
    pub day_index: i64,
    pub gpfalnjhlca: i64,
    pub ggecdfkamji: i64,
    pub fpkahnlmplg: i64,
    pub ellhnfjfkan: i64,
    pub gpglfopjkbh: Vec<i64>,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    pub ndjdbinocpd: Vec<i64>,
    pub ljajmkjgkam: i64,
    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
}
