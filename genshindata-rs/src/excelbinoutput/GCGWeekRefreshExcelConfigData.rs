/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgWeekRefreshExcelConfigData = Vec<GcgWeekRefreshExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgWeekRefreshExcelConfigDatum {
    pub cieagklchjk: i64,
    pub omaphbjepnl: i64,
    pub ppecghpkapm: i64,
    pub fmncgboghph: Vec<Fmncgboghph>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fmncgboghph {
    #[serde(rename = "CHJLNJAMCJA")]
    pub chjlnjamcja: Vec<i64>,
    pub weight: Option<i64>,
}
