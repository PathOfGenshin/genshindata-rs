/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReviseLevelGrowExcelConfigData = Vec<ReviseLevelGrowExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviseLevelGrowExcelConfigDatum {
    pub id: i64,
    pub grade: Vec<i64>,
}
