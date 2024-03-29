/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FungusCampExcelConfigData = Vec<FungusCampExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FungusCampExcelConfigDatum {
    #[serde(rename = "LAJBAJEKBDJ")]
    pub lajbajekbdj: i64,
    pub camp_name_text_map_hash: i64,
    pub unlock_day: i64,
    pub group_link_id: i64,
    pub quest_id: Option<i64>,
    #[serde(rename = "EDAOMDACHPK")]
    pub edaomdachpk: Option<i64>,
    pub watcher_ids: Vec<i64>,
    #[serde(rename = "FDBANLIMMGO")]
    pub fdbanlimmgo: Option<i64>,
}
