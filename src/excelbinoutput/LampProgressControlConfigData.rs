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

pub type LampProgressControlConfigData = Vec<LampProgressControlConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LampProgressControlConfigDatum {
    #[serde(rename = "hour")]
    pub hour: i64,

    #[serde(rename = "minProgress")]
    pub min_progress: i64,

    #[serde(rename = "maxProgress")]
    pub max_progress: i64,
}
