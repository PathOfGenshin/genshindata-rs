/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BartenderAffixExcelConfigData = Vec<BartenderAffixExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BartenderAffixExcelConfigDatum {
    pub id: i64,
    pub order_type: String,
    pub material_id: i64,
    pub material_count: i64,
}
