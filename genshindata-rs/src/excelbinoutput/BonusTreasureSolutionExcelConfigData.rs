/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BonusTreasureSolutionExcelConfigData = Vec<BonusTreasureSolutionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BonusTreasureSolutionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub iifjfkcopaj: f64,
    pub nhnkpdekljf: Vec<i64>,
}
