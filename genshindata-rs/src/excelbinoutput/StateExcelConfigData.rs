/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type StateExcelConfigData = Vec<StateExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateExcelConfigDatum {
    pub state_name: String,
    pub state_type: Option<String>,
    pub rank: Option<i64>,
}
