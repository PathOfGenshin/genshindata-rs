/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type PitchPotStageExcelConfigData = Vec<PitchPotStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PitchPotStageExcelConfigDatum {
    pub stage_id: i64,
    pub gallery_id: i64,
    pub title_text_map_hash: i64,
    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
    pub image_name_hash: f64,
    #[serde(rename = "HKDPBMHJABP")]
    pub hkdpbmhjabp: Vec<i64>,
    #[serde(rename = "GBJCOLBPCAF")]
    pub gbjcolbpcaf: Vec<i64>,
    pub draft_id: i64,
    #[serde(rename = "matchID")]
    pub match_id: i64,
    pub open_day: i64,
    #[serde(rename = "DLJGAJIGPMD")]
    pub dljgajigpmd: i64,
}