/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySalesmanExcelConfigData = Vec<ActivitySalesmanExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySalesmanExcelConfigDatum {
    pub schedule_id: i64,
    pub daily_config_id_list: Vec<i64>,
    pub normal_reward_id_list: Vec<i64>,
    pub special_reward_id_list: Vec<i64>,
    pub special_prob_list: Vec<f64>,
    pub special_reward: SpecialReward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialReward {
    pub obtain_param: String,
    pub id: i64,
    pub reward_type: Option<String>,
    pub obtain_method: Option<String>,
    pub preview_id: Option<i64>,
}
