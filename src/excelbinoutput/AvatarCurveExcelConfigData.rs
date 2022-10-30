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

pub type AvatarCurveExcelConfigData = Vec<AvatarCurveExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarCurveExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "curveInfos")]
    pub curve_infos: Vec<CurveInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct CurveInfo {
    #[serde(rename = "type")]
    pub curve_info_type: Type,

    #[serde(rename = "arith")]
    pub arith: Arith,

    #[serde(rename = "value")]
    pub value: f64,
}

#[derive(Serialize, Deserialize)]
pub enum Arith {
    #[serde(rename = "ARITH_MULTI")]
    ArithMulti,
}

#[derive(Serialize, Deserialize)]
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
