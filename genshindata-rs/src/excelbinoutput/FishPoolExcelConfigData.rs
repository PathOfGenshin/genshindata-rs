/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type FishPoolExcelConfigData = Vec<FishPoolExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FishPoolExcelConfigDatum {
    pub id: i64,
    pub stock_list: Vec<i64>,
    pub stock_guarantee: HashMap<String, i64>,
    pub stock_limit_list: Vec<StockLimitList>,
    pub max_num: i64,
    pub pool_name_text_map_hash: i64,
    pub pool_desc_text_map_hash: i64,
    pub ability_group: AbilityGroup,
    pub team_ability_group: TeamAbilityGroup,
    pub drop_id_list: Vec<i64>,
    pub exclude_fish: Vec<i64>,
    pub daily_limit_num: Option<i64>,
    pub city_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityGroup {
    #[serde(rename = "Avatar_Fishing")]
    AvatarFishing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StockLimitList {
    pub bohpihhnaja: Option<Bohpihhnaja>,
    pub fancihecdbi: Option<i64>,
    pub jnbbdhfodhh: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Bohpihhnaja {
    #[serde(rename = "FISH_STOCK_TYPE_ANY")]
    FishStockTypeAny,
    #[serde(rename = "FISH_STOCK_TYPE_DAY")]
    FishStockTypeDay,
    #[serde(rename = "FISH_STOCK_TYPE_NIGHT")]
    FishStockTypeNight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamAbilityGroup {
    #[serde(rename = "Avatar_Fishing_OnTeam")]
    AvatarFishingOnTeam,
}
