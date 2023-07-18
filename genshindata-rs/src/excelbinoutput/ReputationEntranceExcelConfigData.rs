/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReputationEntranceExcelConfigData = Vec<ReputationEntranceExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReputationEntranceExcelConfigDatum {
    pub text_id: i64,
    pub entrance_id: String,
    pub city_id: i64,
    pub goods_id_vec: Vec<GoodsIdVec>,
    pub cond_name_vec: Vec<i64>,
    pub name_text_map_hash: i64,
    pub title_text_map_hash: i64,
    pub explain_title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub icon_name: String,
    pub order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodsIdVec {
    #[serde(rename = "type")]
    pub goods_id_vec_type: Option<Type>,
    pub param1: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "REPUTATION_ENTRANCE_COND_QUEST")]
    ReputationEntranceCondQuest,
    #[serde(rename = "REPUTATION_ENTRANCE_COND_REPUTATION_LEVEL")]
    ReputationEntranceCondReputationLevel,
}
