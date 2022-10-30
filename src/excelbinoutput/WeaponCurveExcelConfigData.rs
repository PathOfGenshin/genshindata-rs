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

pub type WeaponCurveExcelConfigData = Vec<WeaponCurveExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WeaponCurveExcelConfigDatum {
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
    #[serde(rename = "GROW_CURVE_ATTACK_101")]
    GrowCurveAttack101,

    #[serde(rename = "GROW_CURVE_ATTACK_102")]
    GrowCurveAttack102,

    #[serde(rename = "GROW_CURVE_ATTACK_103")]
    GrowCurveAttack103,

    #[serde(rename = "GROW_CURVE_ATTACK_104")]
    GrowCurveAttack104,

    #[serde(rename = "GROW_CURVE_ATTACK_105")]
    GrowCurveAttack105,

    #[serde(rename = "GROW_CURVE_ATTACK_201")]
    GrowCurveAttack201,

    #[serde(rename = "GROW_CURVE_ATTACK_202")]
    GrowCurveAttack202,

    #[serde(rename = "GROW_CURVE_ATTACK_203")]
    GrowCurveAttack203,

    #[serde(rename = "GROW_CURVE_ATTACK_204")]
    GrowCurveAttack204,

    #[serde(rename = "GROW_CURVE_ATTACK_205")]
    GrowCurveAttack205,

    #[serde(rename = "GROW_CURVE_ATTACK_301")]
    GrowCurveAttack301,

    #[serde(rename = "GROW_CURVE_ATTACK_302")]
    GrowCurveAttack302,

    #[serde(rename = "GROW_CURVE_ATTACK_303")]
    GrowCurveAttack303,

    #[serde(rename = "GROW_CURVE_ATTACK_304")]
    GrowCurveAttack304,

    #[serde(rename = "GROW_CURVE_ATTACK_305")]
    GrowCurveAttack305,

    #[serde(rename = "GROW_CURVE_CRITICAL_101")]
    GrowCurveCritical101,

    #[serde(rename = "GROW_CURVE_CRITICAL_201")]
    GrowCurveCritical201,

    #[serde(rename = "GROW_CURVE_CRITICAL_301")]
    GrowCurveCritical301,
}
