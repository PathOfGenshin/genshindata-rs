/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FishProficientExcelConfigData = Vec<FishProficientExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FishProficientExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "DNKEKOLNNJA")]
    pub dnkekolnnja: Vec<Dnkekolnnja>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Dnkekolnnja {
    pub epakeacleei: i64,
    pub ompjmimdbjh: i64,
    pub hbmmhfhhpec: i64,
}
