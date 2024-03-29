/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ExpeditionDataExcelConfigData = Vec<ExpeditionDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpeditionDataExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,
    pub name_text_map_hash: i64,
    pub city_id: i64,
    pub open_condition_vec: Vec<OpenConditionVec>,
    pub time_reward_vec: Vec<TimeRewardVec>,
    pub description_text_map_hash: i64,
    pub picture: String,
    pub pos_x: f64,
    pub pos_y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenConditionVec {
    pub para: Option<i64>,
    #[serde(rename = "type")]
    pub open_condition_vec_type: Option<Type>,
    pub para2: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "EXP_OPEN_COND_POINT")]
    ExpOpenCondPoint,
    #[serde(rename = "EXP_OPEN_COND_QUEST")]
    ExpOpenCondQuest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRewardVec {
    pub htime: i64,
    pub reward_drop_id: i64,
    pub reward_preview: i64,
}
