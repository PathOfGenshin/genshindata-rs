/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GravenInnocenceCarveStageExcelConfigData = Vec<GravenInnocenceCarveStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GravenInnocenceCarveStageExcelConfigDatum {
    pub id: i64,
    pub carve_add_count: i64,
    pub watcher_id: i64,
    pub open_day: i64,
}
