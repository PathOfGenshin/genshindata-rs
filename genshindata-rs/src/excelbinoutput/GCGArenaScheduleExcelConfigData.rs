/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgArenaScheduleExcelConfigData = Vec<GcgArenaScheduleExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcgArenaScheduleExcelConfigDatum {
    pub schedule_id: i64,
    pub start_time: String,
    pub end_time: String,
    #[serde(rename = "NNHNJCOEKBL")]
    pub nnhnjcoekbl: i64,
}
