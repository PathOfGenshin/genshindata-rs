/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type InstableSprayStageExcelConfigData = Vec<InstableSprayStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstableSprayStageExcelConfigDatum {
    pub stage_id: i64,
    pub open_day: i64,
    #[serde(rename = "KLPBPLKEDKM")]
    pub klpbplkedkm: i64,
    #[serde(rename = "DPNDLLOMALE")]
    pub dpndllomale: i64,
    pub dungeon_id: i64,
    #[serde(rename = "GIKGNBLPBCM")]
    pub gikgnblpbcm: Vec<i64>,
    pub watcher_list: Vec<i64>,
}
