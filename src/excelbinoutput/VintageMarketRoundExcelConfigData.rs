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

pub type VintageMarketRoundExcelConfigData = Vec<VintageMarketRoundExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketRoundExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "DILEKMMNPKG")]
    pub dilekmmnpkg: Vec<Dilekmmnpkg>,
}

#[derive(Serialize, Deserialize)]
pub struct Dilekmmnpkg {
    #[serde(rename = "KLELKECAFFA")]
    pub klelkecaffa: Vec<f64>,

    #[serde(rename = "FCADGBJBNLN")]
    pub fcadgbjbnln: i64,

    #[serde(rename = "IDNLIGANAEG")]
    pub idnliganaeg: f64,

    #[serde(rename = "OJDJDKKNCLN")]
    pub ojdjdkkncln: Option<i64>,

    #[serde(rename = "JLEKNGJJAML")]
    pub jlekngjjaml: f64,
}
