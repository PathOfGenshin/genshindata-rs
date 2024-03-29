/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DailyTaskRewardExcelConfigData = Vec<DailyTaskRewardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DailyTaskRewardExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "dropVec")]
    pub drop_vec: Vec<DropVec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropVec {
    pub drop_id: i64,
    pub preview_reward_id: i64,
}
