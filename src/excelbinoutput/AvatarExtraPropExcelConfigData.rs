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

pub type AvatarExtraPropExcelConfigData = Vec<AvatarExtraPropExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarExtraPropExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "OEFBOCBFJIL")]
    pub oefbocbfjil: Vec<Oefbocbfjil>,

    #[serde(rename = "BBBKPFODPKN")]
    pub bbbkpfodpkn: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Oefbocbfjil {
    #[serde(rename = "propType")]
    pub prop_type: Option<String>,

    #[serde(rename = "propValue")]
    pub prop_value: Option<i64>,
}
