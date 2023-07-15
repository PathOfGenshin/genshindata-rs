/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ExpeditionDifficultyExcelConfigData = Vec<ExpeditionDifficultyExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpeditionDifficultyExcelConfigDatum {
    pub id: i64,
    pub need_hours: i64,
    pub min_player: i64,
    pub max_player: i64,
    pub min_refresh_count: i64,
    pub max_refresh_count: i64,
    pub coef: f64,
}
