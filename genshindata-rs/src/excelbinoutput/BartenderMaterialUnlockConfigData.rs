/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BartenderMaterialUnlockConfigData = Vec<BartenderMaterialUnlockConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BartenderMaterialUnlockConfigDatum {
    pub id: i64,
    #[serde(rename = "LDIADLIENFH")]
    pub ldiadlienfh: i64,
    pub weight: i64,
    #[serde(rename = "OJJJIIEHGNO")]
    pub ojjjiiehgno: i64,
}
