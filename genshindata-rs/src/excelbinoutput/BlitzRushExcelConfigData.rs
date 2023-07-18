/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BlitzRushExcelConfigData = Vec<BlitzRushExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlitzRushExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    pub content_duration: i64,
    pub dungeon_id: i64,
    #[serde(rename = "AGLAOJKGLJI")]
    pub aglaojkglji: i64,
    #[serde(rename = "JLCJBBMHIFH")]
    pub jlcjbbmhifh: i64,
    #[serde(rename = "HEIKKEDPOGO")]
    pub heikkedpogo: i64,
    #[serde(rename = "LMHPDIOMGEL")]
    pub lmhpdiomgel: Vec<i64>,
    pub reward_preview: i64,
}
