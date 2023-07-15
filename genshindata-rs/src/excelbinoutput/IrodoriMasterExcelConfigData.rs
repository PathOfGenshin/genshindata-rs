/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriMasterExcelConfigData = Vec<IrodoriMasterExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IrodoriMasterExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "levelID")]
    pub level_id: i64,
    pub fpecffpfibi: Fpecffpfibi,
    pub gkgnfpbafjh: i64,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    pub mmfjhpahgcn: i64,
    pub gifdffpilge: i64,
    #[serde(rename = "condID")]
    pub cond_id: i64,
    pub bfddpphkgce: i64,
    pub ibcfmhemodk: i64,
    pub oilkkpolheh: i64,
    pub jaejpolkhlo: i64,
    pub dfmcgbnjcka: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Fpecffpfibi {
    #[serde(rename = "IRODORI_MASTER_LEVEL_HARD")]
    IrodoriMasterLevelHard,
    #[serde(rename = "IRODORI_MASTER_LEVEL_MASTER")]
    IrodoriMasterLevelMaster,
    #[serde(rename = "IRODORI_MASTER_LEVEL_NORAML")]
    IrodoriMasterLevelNoraml,
}
