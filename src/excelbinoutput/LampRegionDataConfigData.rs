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

pub type LampRegionDataConfigData = Vec<LampRegionDataConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LampRegionDataConfigDatum {
    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "factor")]
    pub factor: i64,
}
