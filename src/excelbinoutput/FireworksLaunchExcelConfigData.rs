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

pub type FireworksLaunchExcelConfigData = Vec<FireworksLaunchExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FireworksLaunchExcelConfigDatum {
    #[serde(rename = "DNLPOBNFEAN")]
    pub dnlpobnfean: String,

    #[serde(rename = "defaultValue")]
    pub default_value: i64,

    #[serde(rename = "BMIFJGNKLAK")]
    pub bmifjgnklak: Vec<i64>,

    #[serde(rename = "OPFBCDPMAGL")]
    pub opfbcdpmagl: i64,

    #[serde(rename = "CIHGFHICDMG")]
    pub cihgfhicdmg: i64,
}
