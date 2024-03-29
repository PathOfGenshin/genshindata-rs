/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FindHilichurlHiliWeiExcelConfigData = Vec<FindHilichurlHiliWeiExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindHilichurlHiliWeiExcelConfigDatum {
    pub id: i64,
    pub day_index: i64,
    pub duration_hint: String,
    pub group_id: i64,
    pub position_center: Vec<f64>,
    pub position_radius: i64,
    #[serde(rename = "watcherID")]
    pub watcher_id: i64,
}
