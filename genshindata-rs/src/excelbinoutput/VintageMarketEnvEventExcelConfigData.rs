/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketEnvEventExcelConfigData = Vec<VintageMarketEnvEventExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VintageMarketEnvEventExcelConfigDatum {
    pub id: i64,
    pub duration: i64,
    pub effect_list: Vec<EffectList>,
    #[serde(rename = "DCABCCKLBKG")]
    pub dcabccklbkg: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectList {
    #[serde(rename = "type")]
    pub effect_list_type: Option<String>,
    pub param: String,
}
