/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RandomCompoundDisplayExcelConfigData = Vec<RandomCompoundDisplayExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomCompoundDisplayExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "GKDPGHFDGCB")]
    pub gkdpghfdgcb: Vec<Gkdpghfdgcb>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gkdpghfdgcb {
    pub id: Option<i64>,
    pub count: Option<i64>,
}
