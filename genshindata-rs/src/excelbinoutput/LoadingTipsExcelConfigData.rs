/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LoadingTipsExcelConfigData = Vec<LoadingTipsExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadingTipsExcelConfigDatum {
    pub id: i64,
    pub tips_title_text_map_hash: i64,
    pub tips_desc_text_map_hash: i64,
    #[serde(rename = "stageID")]
    pub stage_id: String,
    pub start_time: String,
    pub end_time: String,
    pub limit_open_state: LimitOpenState,
    pub pre_main_quest_ids: String,
    #[serde(rename = "JJILHHPGHHK")]
    pub jjilhhpghhk: Vec<i64>,
    #[serde(rename = "HOBLJBCCKGB")]
    pub hobljbcckgb: Vec<i64>,
    #[serde(rename = "ENBBKNDOMAN")]
    pub enbbkndoman: Vec<Option<serde_json::Value>>,
    pub weight: i64,
    pub min_level: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LimitOpenState {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "OPEN_STATE_COMBINE")]
    OpenStateCombine,
    #[serde(rename = "OPEN_STATE_GACHA")]
    OpenStateGacha,
    #[serde(rename = "OPEN_STATE_GUIDE_BLOSSOM")]
    OpenStateGuideBlossom,
    #[serde(rename = "OPEN_STATE_LOADINGTIPS_ENKANOMIYA")]
    OpenStateLoadingtipsEnkanomiya,
}
