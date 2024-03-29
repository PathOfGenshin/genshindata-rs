/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ShareCdExcelConfigData = Vec<ShareCdExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareCdExcelConfigDatum {
    pub id: i64,
    pub max_charge_num: i64,
    pub cool_down_list: Vec<CoolDownList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoolDownList {
    pub cool_down_time: f64,
    pub token: Option<i64>,
}
