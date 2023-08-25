/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GadgetPropExcelConfigData = Vec<GadgetPropExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GadgetPropExcelConfigDatum {
    pub id: i64,
    pub hp: f64,
    pub hp_curve: Curve,
    pub attack: Option<f64>,
    pub attack_curve: AttackCurve,
    pub defense: Option<i64>,
    pub defense_curve: Curve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttackCurve {
    #[serde(rename = "GROW_CURVE_ATTACK")]
    GrowCurveAttack,
    #[serde(rename = "GROW_CURVE_HP")]
    GrowCurveHp,
    #[serde(rename = "GROW_CURVE_TOWERATK")]
    GrowCurveToweratk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Curve {
    #[serde(rename = "GROW_CURVE_DEFENDING")]
    GrowCurveDefending,
    #[serde(rename = "GROW_CURVE_DEFENSE")]
    GrowCurveDefense,
    #[serde(rename = "GROW_CURVE_HP")]
    GrowCurveHp,
    #[serde(rename = "GROW_CURVE_HP_ENVIRONMENT")]
    GrowCurveHpEnvironment,
}
