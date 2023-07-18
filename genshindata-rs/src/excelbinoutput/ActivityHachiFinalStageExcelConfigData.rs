/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityHachiFinalStageExcelConfigData = Vec<ActivityHachiFinalStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityHachiFinalStageExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "stageId")]
    pub stage_id: i64,
    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,
    #[serde(rename = "EEEAIOEAEDC")]
    pub eeeaioeaedc: i64,
    #[serde(rename = "IJCEHGBANBF")]
    pub ijcehgbanbf: i64,
    #[serde(rename = "IDMHLDKDFBJ")]
    pub idmhldkdfbj: i64,
    #[serde(rename = "JMOAEBINIMO")]
    pub jmoaebinimo: i64,
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,
    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,
    #[serde(rename = "openDay")]
    pub open_day: i64,
    #[serde(rename = "JCDOCEIEHOK")]
    pub jcdoceiehok: Vec<i64>,
    #[serde(rename = "AGLAOJKGLJI")]
    pub aglaojkglji: i64,
}
