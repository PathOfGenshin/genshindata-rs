/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SorushTrialStageExcelConfigData = Vec<SorushTrialStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SorushTrialStageExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,
    pub bkoiogdhknb: i64,
    pub bdoknjhfpln: Vec<i64>,
    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,
    pub maoklamadhe: Option<i64>,
}
