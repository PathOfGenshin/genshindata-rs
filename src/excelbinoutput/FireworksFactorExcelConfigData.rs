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

pub type FireworksFactorExcelConfigData = Vec<FireworksFactorExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FireworksFactorExcelConfigDatum {
    #[serde(rename = "IMGGODAFPAE")]
    pub imggodafpae: String,

    #[serde(rename = "PCANKJOPFIE")]
    pub pcankjopfie: i64,

    #[serde(rename = "MMILBCNCHJI")]
    pub mmilbcnchji: String,

    #[serde(rename = "JKBKMKPGJBB")]
    pub jkbkmkpgjbb: i64,

    #[serde(rename = "NLAJBKLHNPO")]
    pub nlajbklhnpo: i64,

    #[serde(rename = "JIFAHOIFLFK")]
    pub jifahoiflfk: i64,

    #[serde(rename = "FKNMKCFPIFI")]
    pub fknmkcfpifi: f64,
}
