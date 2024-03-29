/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySummerTimeFloatSignalExcelConfigData = Vec<ActivitySummerTimeFloatSignalExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySummerTimeFloatSignalExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub group_id: i64,
    pub config_id: i64,
    pub mist_zone_id: Option<i64>,
    pub vehicle_config_id: i64,
    pub vehicle_gadget_id: i64,
    pub is_transfer_anchor: Option<bool>,
}
