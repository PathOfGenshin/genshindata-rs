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

pub type MichiaeOfferingDataExcelConfigData = Vec<MichiaeOfferingDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MichiaeOfferingDataExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "OIPFAHBAMNA")]
    pub oipfahbamna: i64,

    #[serde(rename = "BNLGCJBHMOL")]
    pub bnlgcjbhmol: Vec<String>,

    #[serde(rename = "KEDMKFBECCK")]
    pub kedmkfbecck: i64,

    #[serde(rename = "IDDFPFFACDO")]
    pub iddfpffacdo: Vec<String>,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}
