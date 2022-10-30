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
use std::collections::HashMap;

pub type GatherBundleExcelConfigData = Vec<GatherBundleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GatherBundleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "bundleName")]
    pub bundle_name: String,

    #[serde(rename = "baseGadgetId")]
    pub base_gadget_id: i64,

    #[serde(rename = "points")]
    pub points: Vec<HashMap<String, f64>>,
}
