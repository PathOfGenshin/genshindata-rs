/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FishRodExcelConfigData = Vec<FishRodExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FishRodExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub kcldlonamcg: i64,
    #[serde(rename = "cityId")]
    pub city_id: Option<i64>,
    pub lmgbonlbmon: Option<f64>,
    pub ncjdohcfmma: Option<i64>,
    pub gkomngiomin: Option<i64>,
}
