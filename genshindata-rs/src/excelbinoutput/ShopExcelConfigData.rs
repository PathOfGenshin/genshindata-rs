/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ShopExcelConfigData = Vec<ShopExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopExcelConfigDatum {
    pub shop_id: i64,
    pub shop_type: String,
    pub open_state_type: Option<String>,
    pub refresh_type: Option<String>,
    pub refresh_param: Option<i64>,
    pub city_id: Option<i64>,
    pub city_discount_level: Option<i64>,
    pub scoin_discount_rate: Option<i64>,
    #[serde(rename = "vipFuncID")]
    pub vip_func_id: Option<i64>,
    #[serde(rename = "DDDFKNGIMFL")]
    pub dddfkngimfl: Option<bool>,
    #[serde(rename = "EPEAPGAJLLC")]
    pub epeapgajllc: Option<bool>,
}
