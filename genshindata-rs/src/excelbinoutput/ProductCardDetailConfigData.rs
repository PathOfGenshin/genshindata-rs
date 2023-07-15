/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ProductCardDetailConfigData = Vec<ProductCardDetailConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductCardDetailConfigDatum {
    pub card_product_type: String,
    pub item_name_text_map_hash: i64,
    pub icon: String,
    pub total_limit_days: i64,
    pub days: i64,
    pub hcoin_per_day: i64,
    pub mcoin_base: i64,
    pub base_item_map: ItemMap,
    pub per_day_item_map: ItemMap,
    pub replace_mcoin_num: i64,
    pub first_reward_text_text_map_hash: i64,
    pub daily_reward_text_text_map_hash: i64,
    pub total_reward_text_text_map_hash: i64,
    pub total_days_text_text_map_hash: i64,
    pub remain_days_text_text_map_hash: i64,
    pub remain_days_text2_text_map_hash: i64,
    pub explain_title_text_map_hash: i64,
    pub explain_desc_text_map_hash: i64,
    pub sort_level: i64,
    pub config_id: i64,
    pub price_tier: String,
    pub shop_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMap {
}
