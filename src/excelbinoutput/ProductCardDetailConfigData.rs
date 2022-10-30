// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type ProductCardDetailConfigData = Vec<ProductCardDetailConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ProductCardDetailConfigDatum {
    #[serde(rename = "cardProductType")]
    pub card_product_type: String,

    #[serde(rename = "itemNameTextMapHash")]
    pub item_name_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "totalLimitDays")]
    pub total_limit_days: i64,

    #[serde(rename = "days")]
    pub days: i64,

    #[serde(rename = "hcoinPerDay")]
    pub hcoin_per_day: i64,

    #[serde(rename = "mcoinBase")]
    pub mcoin_base: i64,

    #[serde(rename = "baseItemMap")]
    pub base_item_map: ItemMap,

    #[serde(rename = "perDayItemMap")]
    pub per_day_item_map: ItemMap,

    #[serde(rename = "replaceMcoinNum")]
    pub replace_mcoin_num: i64,

    #[serde(rename = "firstRewardTextTextMapHash")]
    pub first_reward_text_text_map_hash: i64,

    #[serde(rename = "dailyRewardTextTextMapHash")]
    pub daily_reward_text_text_map_hash: i64,

    #[serde(rename = "totalRewardTextTextMapHash")]
    pub total_reward_text_text_map_hash: i64,

    #[serde(rename = "totalDaysTextTextMapHash")]
    pub total_days_text_text_map_hash: i64,

    #[serde(rename = "remainDaysTextTextMapHash")]
    pub remain_days_text_text_map_hash: i64,

    #[serde(rename = "remainDaysText2TextMapHash")]
    pub remain_days_text2_text_map_hash: i64,

    #[serde(rename = "explainTitleTextMapHash")]
    pub explain_title_text_map_hash: i64,

    #[serde(rename = "explainDescTextMapHash")]
    pub explain_desc_text_map_hash: i64,

    #[serde(rename = "sortLevel")]
    pub sort_level: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "priceTier")]
    pub price_tier: String,

    #[serde(rename = "shopType")]
    pub shop_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ItemMap {}
