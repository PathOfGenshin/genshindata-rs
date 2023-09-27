/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LunaRiteSearchingExcelConfigData = Vec<LunaRiteSearchingExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LunaRiteSearchingExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub region_type: String,
    pub open_day: i64,
    #[serde(rename = "progressWatcherID")]
    pub progress_watcher_id: i64,
    #[serde(rename = "watcherID")]
    pub watcher_id: Vec<i64>,
    pub region_center: Vec<i64>,
    pub region_name_hash: f64,
    pub region_radius: i64,
    pub chest_cond: i64,
    pub rune_cond: i64,
    pub chest_mark_num: i64,
    pub rune_mark_num: i64,
}
