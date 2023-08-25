/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RogueDiaryDungeonExcelConfigData = Vec<RogueDiaryDungeonExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RogueDiaryDungeonExcelConfigDatum {
    pub dungeon_id: i64,
    pub group_list: Vec<i64>,
    #[serde(rename = "LAILCPHAIKP")]
    pub lailcphaikp: Vec<Lailcphaikp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Lailcphaikp {
    pub aheocjaocfa: Option<i64>,
    pub cmfidkjfkki: Vec<i64>,
    pub plheneoflnm: Option<i64>,
}
