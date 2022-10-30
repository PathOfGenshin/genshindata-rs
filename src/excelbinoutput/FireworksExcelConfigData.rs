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

pub type FireworksExcelConfigData = Vec<FireworksExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FireworksExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "MFCAENJKICP")]
    pub mfcaenjkicp: Vec<Mfcaenjkicp>,

    #[serde(rename = "EDOLACPCHJG")]
    pub edolacpchjg: Edolacpchjg,

    #[serde(rename = "BCCACAJHPPF")]
    pub bccacajhppf: Bccacajhppf,

    #[serde(rename = "AKEJMLEELLC")]
    pub akejmleellc: String,

    #[serde(rename = "HHFBDJPPMOP")]
    pub hhfbdjppmop: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Mfcaenjkicp {
    #[serde(rename = "type")]
    pub mfcaenjkicp_type: Type,

    #[serde(rename = "AFLHGCMHDMK")]
    pub aflhgcmhdmk: i64,

    #[serde(rename = "CLIOCIMNIAL")]
    pub cliocimnial: Option<bool>,

    #[serde(rename = "AHENCBKOPAO")]
    pub ahencbkopao: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Bccacajhppf {
    #[serde(rename = "Eff_SceneObj_Fireworks_Bullet")]
    EffSceneObjFireworksBullet,
}

#[derive(Serialize, Deserialize)]
pub enum Edolacpchjg {
    #[serde(rename = "PatternShapeFireworks")]
    PatternShapeFireworks,

    #[serde(rename = "SphericalFireworks")]
    SphericalFireworks,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FIREWORKS_REFORM_PARAM_COLOR")]
    FireworksReformParamColor,

    #[serde(rename = "FIREWORKS_REFORM_PARAM_DENSITY")]
    FireworksReformParamDensity,

    #[serde(rename = "FIREWORKS_REFORM_PARAM_HEIGHT")]
    FireworksReformParamHeight,

    #[serde(rename = "FIREWORKS_REFORM_PARAM_ROTATION")]
    FireworksReformParamRotation,

    #[serde(rename = "FIREWORKS_REFORM_PARAM_SIZE")]
    FireworksReformParamSize,
}
