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

pub type MichiaeErosionMapExcelConfigData = Vec<MichiaeErosionMapExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MichiaeErosionMapExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "GIPBMPPGNAH")]
    pub gipbmppgnah: Vec<i64>,

    #[serde(rename = "OADMMOBEMEF")]
    pub oadmmobemef: i64,

    #[serde(rename = "ANHIAMHEDIA")]
    pub anhiamhedia: i64,
}
