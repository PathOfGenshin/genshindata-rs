/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgPickGroupExcelConfigData = Vec<GcgPickGroupExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcgPickGroupExcelConfigDatum {
    pub id: i64,
    pub character_id: i64,
    #[serde(rename = "NNEDPFCKCIN")]
    pub nnedpfckcin: Vec<i64>,
}
