/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketBargainExcelConfigData = Vec<VintageMarketBargainExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct VintageMarketBargainExcelConfigDatum {
    pub oeogfelfnod: i64,
    #[serde(rename = "questId")]
    pub quest_id: i64,
    pub heiflgobhpf: i64,
    pub hmamkcboljc: i64,
}
