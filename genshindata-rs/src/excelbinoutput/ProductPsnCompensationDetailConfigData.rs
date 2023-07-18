/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ProductPsnCompensationDetailConfigData = Vec<ProductPsnCompensationDetailConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductPsnCompensationDetailConfigDatum {
    pub content_vec: Vec<ContentVec>,
    pub mail_config_id: i64,
    #[serde(rename = "EDCMPNAMJJM")]
    pub edcmpnamjjm: i64,
    pub config_id: i64,
    pub price_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentVec {
    pub material_id: Option<i64>,
    pub material_num: Option<i64>,
}
