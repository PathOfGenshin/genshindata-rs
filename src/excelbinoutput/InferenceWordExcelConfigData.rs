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

pub type InferenceWordExcelConfigData = Vec<InferenceWordExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InferenceWordExcelConfigDatum {
    #[serde(rename = "ELABMBFNIFF")]
    pub elabmbfniff: i64,

    #[serde(rename = "DBLKEGDILKA")]
    pub dblkegdilka: i64,

    #[serde(rename = "MIBDGELFAGJ")]
    pub mibdgelfagj: Option<bool>,

    #[serde(rename = "NEDNACNFACN")]
    pub nednacnfacn: Option<bool>,

    #[serde(rename = "EMPJPMNEIEN")]
    pub empjpmneien: Vec<Djhjhpnpnno>,

    #[serde(rename = "CECCKJKILKP")]
    pub cecckjkilkp: Option<bool>,

    #[serde(rename = "PCDLIMAIIDP")]
    pub pcdlimaiidp: Option<i64>,

    #[serde(rename = "DJHJHPNPNNO")]
    pub djhjhpnpnno: Vec<Djhjhpnpnno>,

    #[serde(rename = "OPLJFLJDOPL")]
    pub opljfljdopl: i64,

    #[serde(rename = "LLCPCAJANNN")]
    pub llcpcajannn: Option<i64>,

    #[serde(rename = "LCPLAOIPCEO")]
    pub lcplaoipceo: Option<i64>,

    #[serde(rename = "NIACGIJOKHM")]
    pub niacgijokhm: Option<i64>,

    #[serde(rename = "DGFAADLELCM")]
    pub dgfaadlelcm: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Djhjhpnpnno {
    #[serde(rename = "type")]
    pub djhjhpnpnno_type: Option<String>,

    #[serde(rename = "param")]
    pub param: String,
}
