/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ShopRotateExcelConfigData = Vec<ShopRotateExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopRotateExcelConfigDatum {
    pub id: i64,
    pub rotate_id: i64,
    pub item_id: i64,
    pub rotate_order: i64,
}
