/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AkaFesArchitectLevelExcelConfigData = Vec<AkaFesArchitectLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AkaFesArchitectLevelExcelConfigDatum {
    pub id: i64,
    pub open_day: i64,
    pub title_text_map_hash: i64,
    #[serde(rename = "BEOKKFMBNFP")]
    pub beokkfmbnfp: Vec<String>,
    pub desc_text_map_hash: i64,
    #[serde(rename = "BFHNEIDKEHA")]
    pub bfhneidkeha: Vec<i64>,
    #[serde(rename = "GCBEBGGCOOK")]
    pub gcbebggcook: Vec<String>,
    pub dungeon_id: i64,
    pub watcher_list: Vec<i64>,
}
