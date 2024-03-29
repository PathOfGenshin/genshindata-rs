/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ProductMcoinDetailConfigData = Vec<ProductMcoinDetailConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductMcoinDetailConfigDatum {
    pub item_name_text_map_hash: i64,
    pub prim_name_text_map_hash: i64,
    pub icon: String,
    pub mcoin_num: i64,
    pub mcoin_first: i64,
    pub seqence: i64,
    pub config_id: i64,
    pub price_tier: String,
    pub shop_type: String,
    pub mcoin_non_first: Option<i64>,
}
