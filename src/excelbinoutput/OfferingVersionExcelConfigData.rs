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

pub type OfferingVersionExcelConfigData = Vec<OfferingVersionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OfferingVersionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "offeringId")]
    pub offering_id: i64,

    #[serde(rename = "OFLEKJIGNHC")]
    pub oflekjignhc: i64,
}
