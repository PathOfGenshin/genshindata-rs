/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FlightActivityMedalExcelConfigData = Vec<FlightActivityMedalExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightActivityMedalExcelConfigDatum {
    pub id: i64,
    pub medal_icon: String,
    pub daily_info: Vec<DailyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyInfo {
    pub watcher: i64,
}
