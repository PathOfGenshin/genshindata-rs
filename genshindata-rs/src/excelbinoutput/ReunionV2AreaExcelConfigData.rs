/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReunionV2AreaExcelConfigData = Vec<ReunionV2AreaExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ReunionV2AreaExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "priority")]
    pub priority: i64,
    pub lnoigblcfma: String,
    pub appkocmcfjj: String,
    pub bcpolebaibd: String,
    pub oidehfhbbfj: String,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    #[serde(rename = "markID")]
    pub mark_id: i64,
    #[serde(rename = "areaNameTextMapHash")]
    pub area_name_text_map_hash: i64,
    pub bjekjogcnnl: bool,
    pub egbfepjmpco: i64,
}
