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
    pub piljkjocijj: Vec<Piljkjocijj>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Piljkjocijj {
    pub id: Option<i64>,
    #[serde(rename = "IKCCKNLEDBO")]
    pub ikccknledbo: Option<i64>,
    pub num: Option<i64>,
}
