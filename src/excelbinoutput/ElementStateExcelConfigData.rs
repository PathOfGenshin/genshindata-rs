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

pub type ElementStateExcelConfigData = Vec<ElementStateExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ElementStateExcelConfigDatum {
    #[serde(rename = "elementType")]
    pub element_type: String,

    #[serde(rename = "elementIcon")]
    pub element_icon: String,

    #[serde(rename = "rank")]
    pub rank: i64,
}
