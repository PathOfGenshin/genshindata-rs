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
    pub amidlbbfihh: Amidlbbfihh,
    pub ldiadlienfh: i64,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    pub megjppdhaeh: i64,
    pub plgoipgmbci: i64,
    #[serde(rename = "condID")]
    pub cond_id: i64,
    pub gegjjibkigl: i64,
    pub jeadjlieaen: i64,
    pub kafdhmmiebj: i64,
    pub emodlhglmkg: i64,
    pub lacaabdbkfk: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Amidlbbfihh {
    #[serde(rename = "IRODORI_MASTER_LEVEL_HARD")]
    IrodoriMasterLevelHard,
    #[serde(rename = "IRODORI_MASTER_LEVEL_MASTER")]
    IrodoriMasterLevelMaster,
    #[serde(rename = "IRODORI_MASTER_LEVEL_NORAML")]
    IrodoriMasterLevelNoraml,
}
