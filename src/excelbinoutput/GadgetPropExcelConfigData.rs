// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type GadgetPropExcelConfigData = Vec<GadgetPropExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GadgetPropExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "hp")]
    pub hp: f64,

    #[serde(rename = "hpCurve")]
    pub hp_curve: Curve,

    #[serde(rename = "attack")]
    pub attack: Option<f64>,

    #[serde(rename = "attackCurve")]
    pub attack_curve: AttackCurve,

    #[serde(rename = "defense")]
    pub defense: Option<f64>,

    #[serde(rename = "defenseCurve")]
    pub defense_curve: Curve,
}

#[derive(Serialize, Deserialize)]
pub enum AttackCurve {
    #[serde(rename = "GROW_CURVE_ATTACK")]
    GrowCurveAttack,

    #[serde(rename = "GROW_CURVE_HP")]
    GrowCurveHp,

    #[serde(rename = "GROW_CURVE_TOWERATK")]
    GrowCurveToweratk,
}

#[derive(Serialize, Deserialize)]
pub enum Curve {
    #[serde(rename = "GROW_CURVE_DEFENDING")]
    GrowCurveDefending,

    #[serde(rename = "GROW_CURVE_DEFENSE")]
    GrowCurveDefense,

    #[serde(rename = "GROW_CURVE_HP")]
    GrowCurveHp,
}
