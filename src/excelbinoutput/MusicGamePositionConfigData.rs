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

pub type MusicGamePositionConfigData = Vec<MusicGamePositionConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MusicGamePositionConfigDatum {
    #[serde(rename = "OIACGMOBJNN")]
    pub oiacgmobjnn: i64,

    #[serde(rename = "GDCOCCJDPDI")]
    pub gdcoccjdpdi: Vec<f64>,

    #[serde(rename = "LMNJLDKBOEO")]
    pub lmnjldkboeo: Option<f64>,

    #[serde(rename = "MECLDFABFMF")]
    pub mecldfabfmf: Option<i64>,

    #[serde(rename = "CMPIGIDKDNJ")]
    pub cmpigidkdnj: Option<i64>,

    #[serde(rename = "LOPAGCEFEAG")]
    pub lopagcefeag: Option<i64>,

    #[serde(rename = "NLJCBMIJAKM")]
    pub nljcbmijakm: String,
}
