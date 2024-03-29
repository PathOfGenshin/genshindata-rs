/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SorushTrialStageDetailExcelConfigData = Vec<SorushTrialStageDetailExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SorushTrialStageDetailExcelConfigDatum {
    #[serde(rename = "BHDAKHJDGAA")]
    pub bhdakhjdgaa: i64,
    #[serde(rename = "type")]
    pub sorush_trial_stage_detail_excel_config_datum_type: Type,
    pub ref_id: i64,
    pub tutorial_id: i64,
    #[serde(rename = "ANLLLNOOEHA")]
    pub anlllnooeha: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "SORUSH_TRIAL_STAGE_DETAIL_TYPE_HITMAN")]
    SorushTrialStageDetailTypeHitman,
    #[serde(rename = "SORUSH_TRIAL_STAGE_DETAIL_TYPE_PHOTOMATCH")]
    SorushTrialStageDetailTypePhotomatch,
    #[serde(rename = "SORUSH_TRIAL_STAGE_DETAIL_TYPE_QUEST")]
    SorushTrialStageDetailTypeQuest,
    #[serde(rename = "SORUSH_TRIAL_STAGE_DETAIL_TYPE_RACE")]
    SorushTrialStageDetailTypeRace,
}
