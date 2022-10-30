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
use std::collections::HashMap;

pub type FishPoolExcelConfigData = Vec<FishPoolExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FishPoolExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_stockList")]
    pub stock_list: Vec<i64>,

    #[serde(rename = "_stockGuarantee")]
    pub stock_guarantee: HashMap<String, i64>,

    #[serde(rename = "_stockLimitList")]
    pub stock_limit_list: Vec<StockLimitList>,

    #[serde(rename = "_maxNum")]
    pub max_num: i64,

    #[serde(rename = "_poolNameTextMapHash")]
    pub pool_name_text_map_hash: i64,

    #[serde(rename = "_poolDescTextMapHash")]
    pub pool_desc_text_map_hash: i64,

    #[serde(rename = "_abilityGroup")]
    pub ability_group: AbilityGroup,

    #[serde(rename = "_teamAbilityGroup")]
    pub team_ability_group: TeamAbilityGroup,

    #[serde(rename = "_dropIdList")]
    pub drop_id_list: Vec<i64>,

    #[serde(rename = "_excludeFish")]
    pub exclude_fish: Vec<i64>,

    #[serde(rename = "_dailyLimitNum")]
    pub daily_limit_num: Option<i64>,

    #[serde(rename = "_cityId")]
    pub city_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct StockLimitList {
    #[serde(rename = "PJLPNHLNIPE")]
    pub pjlpnhlnipe: Option<Pjlpnhlnipe>,

    #[serde(rename = "GBPALEDIHCF")]
    pub gbpaledihcf: Option<i64>,

    #[serde(rename = "KJMMGOFLMKG")]
    pub kjmmgoflmkg: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum AbilityGroup {
    #[serde(rename = "Avatar_Fishing")]
    AvatarFishing,
}

#[derive(Serialize, Deserialize)]
pub enum Pjlpnhlnipe {
    #[serde(rename = "FISH_STOCK_TYPE_ANY")]
    FishStockTypeAny,

    #[serde(rename = "FISH_STOCK_TYPE_DAY")]
    FishStockTypeDay,

    #[serde(rename = "FISH_STOCK_TYPE_NIGHT")]
    FishStockTypeNight,
}

#[derive(Serialize, Deserialize)]
pub enum TeamAbilityGroup {
    #[serde(rename = "Avatar_Fishing_OnTeam")]
    AvatarFishingOnTeam,
}
