/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LevelTagGroupsExcelConfigData = Vec<LevelTagGroupsExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LevelTagGroupsExcelConfigDatum {
    pub id: i64,
    pub gnjaijchfdp: Vec<Gnjaijchfdp>,
    pub ddhpdjnlnjc: Vec<i64>,
    pub adbjbmogbhj: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Gnjaijchfdp {
    pub afglionnjnn: Vec<i64>,
}
