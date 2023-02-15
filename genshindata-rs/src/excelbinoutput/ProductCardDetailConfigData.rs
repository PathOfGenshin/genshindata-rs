// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ProductCardDetailConfigData = Vec<ProductCardDetailConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemMap {
}

pub fn load() -> Result<ProductCardDetailConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ProductCardDetailConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
