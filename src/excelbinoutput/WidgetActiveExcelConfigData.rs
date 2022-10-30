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

pub type WidgetActiveExcelConfigData = Vec<WidgetActiveExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WidgetActiveExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "NFKEAHLDMOO")]
    pub nfkeahldmoo: Vec<Option<serde_json::Value>>,

    #[serde(rename = "OEIPJMOBLOB")]
    pub oeipjmoblob: String,

    #[serde(rename = "GLFIAPGPBCM")]
    pub glfiapgpbcm: Vec<i64>,

    #[serde(rename = "PDGMHLFNBFD")]
    pub pdgmhlfnbfd: Vec<i64>,

    #[serde(rename = "GKFJLONCCEO")]
    pub gkfjloncceo: Option<bool>,
}
