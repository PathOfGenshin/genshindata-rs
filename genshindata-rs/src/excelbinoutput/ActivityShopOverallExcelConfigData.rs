/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityShopOverallExcelConfigData = Vec<ActivityShopOverallExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityShopOverallExcelConfigDatum {
    pub schedule_id: i64,
    pub shop_type: String,
    pub sheet_list: Vec<i64>,
}
