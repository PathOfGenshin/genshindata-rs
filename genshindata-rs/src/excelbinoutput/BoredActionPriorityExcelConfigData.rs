/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BoredActionPriorityExcelConfigData = Vec<BoredActionPriorityExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoredActionPriorityExcelConfigDatum {
    pub action_type: String,
    pub weight: Option<i64>,
}
