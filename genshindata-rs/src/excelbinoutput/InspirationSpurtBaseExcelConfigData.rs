/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type InspirationSpurtBaseExcelConfigData = Vec<InspirationSpurtBaseExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct InspirationSpurtBaseExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,
    pub nhkndhcfjhm: i64,
    pub jpiclbfebnl: i64,
}