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

pub type PartnerCurveExcelConfigData = Vec<PartnerCurveExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PartnerCurveExcelConfigDatum {
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
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum Arith {
    #[serde(rename = "ARITH_ADD")]
    ArithAdd,

    #[serde(rename = "ARITH_MULTI")]
    ArithMulti,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "GROW_CURVE_ATTACK")]
    GrowCurveAttack,

    #[serde(rename = "GROW_CURVE_ATTACK_2")]
    GrowCurveAttack2,

    #[serde(rename = "GROW_CURVE_DEFENDING")]
    GrowCurveDefending,

    #[serde(rename = "GROW_CURVE_DEFENSE")]
    GrowCurveDefense,

    #[serde(rename = "GROW_CURVE_ELEMENT")]
    GrowCurveElement,

    #[serde(rename = "GROW_CURVE_HP")]
    GrowCurveHp,

    #[serde(rename = "GROW_CURVE_HP_2")]
    GrowCurveHp2,

    #[serde(rename = "GROW_CURVE_STRIKE")]
    GrowCurveStrike,

    #[serde(rename = "GROW_CURVE_STRIKE_HURT")]
    GrowCurveStrikeHurt,
}
