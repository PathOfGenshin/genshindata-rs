/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GroupTagExcelConfigData = Vec<GroupTagExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTagExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "EJBBKGJHKOO")]
    pub ejbbkgjhkoo: String,
}
