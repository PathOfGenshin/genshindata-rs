/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ToyBattleQteTargetExcelConfigData = Vec<ToyBattleQteTargetExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToyBattleQteTargetExcelConfigDatum {
    #[serde(rename = "JCGLNJIJLLB")]
    pub jcglnjijllb: i64,
    pub score: Option<i64>,
}
