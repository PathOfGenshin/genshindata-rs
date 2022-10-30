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

pub type QteExcelConfigData = Vec<QteExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct QteExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "DKKOLADOEIN")]
    pub dkkoladoein: i64,

    #[serde(rename = "MCPBOFIHJIP")]
    pub mcpbofihjip: String,

    #[serde(rename = "NIHNINEDLJA")]
    pub nihninedlja: Vec<Dappdjjkkne>,

    #[serde(rename = "ONMPEEKOCCF")]
    pub onmpeekoccf: Vec<Onmpeekoccf>,

    #[serde(rename = "DAPPDJJKKNE")]
    pub dappdjjkkne: Vec<Dappdjjkkne>,
}

#[derive(Serialize, Deserialize)]
pub struct Dappdjjkkne {
    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Onmpeekoccf {
    #[serde(rename = "type")]
    pub onmpeekoccf_type: Option<String>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}
