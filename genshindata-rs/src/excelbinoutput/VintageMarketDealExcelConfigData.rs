/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketDealExcelConfigData = Vec<VintageMarketDealExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct VintageMarketDealExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,
    pub nefbhpdoonn: Vec<Nefbhpdoonn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Nefbhpdoonn {
    #[serde(rename = "id")]
    pub id: Option<i64>,
    pub lhhaddhdlcn: Option<i64>,
    pub epakeacleei: Option<i64>,
}
