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

pub type DynamicInteractionExcelConfigData = Vec<DynamicInteractionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DynamicInteractionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "ILIJICIBEPF")]
    pub ilijicibepf: i64,

    #[serde(rename = "EANBGKDLNPD")]
    pub eanbgkdlnpd: i64,

    #[serde(rename = "LDFDOEJKCLG")]
    pub ldfdoejkclg: String,

    #[serde(rename = "BAJEDOOFCII")]
    pub bajedoofcii: i64,

    #[serde(rename = "HDHICCAMMLB")]
    pub hdhiccammlb: String,

    #[serde(rename = "EFFFHNNFJML")]
    pub efffhnnfjml: bool,
}
