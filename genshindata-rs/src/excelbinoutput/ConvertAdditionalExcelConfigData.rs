/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ConvertAdditionalExcelConfigData = Vec<ConvertAdditionalExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ConvertAdditionalExcelConfigDatum {
    #[serde(rename = "combineId")]
    pub combine_id: i64,
    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,
    pub jjocipboaai: i64,
    pub hplhajdemca: String,
}
