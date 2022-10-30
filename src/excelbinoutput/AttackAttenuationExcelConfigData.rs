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

pub type AttackAttenuationExcelConfigData = Vec<AttackAttenuationExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AttackAttenuationExcelConfigDatum {
    #[serde(rename = "JEKKDBBFGEC")]
    pub jekkdbbfgec: String,

    #[serde(rename = "KKIDACKDPCJ")]
    pub kkidackdpcj: f64,

    #[serde(rename = "HFDPHNAEMEF")]
    pub hfdphnaemef: Vec<f64>,

    #[serde(rename = "BLDCNMMEOGJ")]
    pub bldcnmmeogj: Vec<f64>,

    #[serde(rename = "OKKDAJNNIPB")]
    pub okkdajnnipb: Vec<f64>,
}
