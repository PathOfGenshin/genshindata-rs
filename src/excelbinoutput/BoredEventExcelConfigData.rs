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

pub type BoredEventExcelConfigData = Vec<BoredEventExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BoredEventExcelConfigDatum {
    #[serde(rename = "eventType")]
    pub event_type: String,

    #[serde(rename = "param")]
    pub param: i64,

    #[serde(rename = "isEnable")]
    pub is_enable: Option<bool>,

    #[serde(rename = "addBored")]
    pub add_bored: i64,

    #[serde(rename = "maxBored")]
    pub max_bored: i64,
}
