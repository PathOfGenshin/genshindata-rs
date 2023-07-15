/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AvatarCurveExcelConfigData = Vec<AvatarCurveExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarCurveExcelConfigDatum {
    pub level: i64,
    pub curve_infos: Vec<CurveInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveInfo {
    #[serde(rename = "type")]
    pub curve_info_type: Type,
    pub arith: Arith,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Arith {
    #[serde(rename = "ARITH_MULTI")]
    ArithMulti,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "GROW_CURVE_ATTACK_S4")]
    GrowCurveAttackS4,
    #[serde(rename = "GROW_CURVE_ATTACK_S5")]
    GrowCurveAttackS5,
    #[serde(rename = "GROW_CURVE_HP_S4")]
    GrowCurveHpS4,
    #[serde(rename = "GROW_CURVE_HP_S5")]
    GrowCurveHpS5,
}
