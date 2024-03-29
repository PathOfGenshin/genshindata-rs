/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FishProficientExcelConfigData = Vec<FishProficientExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FishProficientExcelConfigDatum {
    pub id: i64,
    pub gear: Vec<Gear>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gear {
    pub num: i64,
    pub in_speed: i64,
    pub out_speed: i64,
}
