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

pub type LevelTagMapAreaConfigData = Vec<LevelTagMapAreaConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LevelTagMapAreaConfigDatum {
    #[serde(rename = "HPJOPNJEKEN")]
    pub hpjopnjeken: i64,

    #[serde(rename = "AKJHAEJHIMG")]
    pub akjhaejhimg: i64,

    #[serde(rename = "index")]
    pub index: Option<i64>,
}
