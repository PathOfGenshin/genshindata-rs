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

pub type GravenInnocenceObjectDataExcelConfigData = Vec<GravenInnocenceObjectDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GravenInnocenceObjectDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "EAJLPCOPPBP")]
    pub eajlpcoppbp: Option<i64>,

    #[serde(rename = "JHNAEFBADBI")]
    pub jhnaefbadbi: i64,

    #[serde(rename = "watcher")]
    pub watcher: i64,

    #[serde(rename = "picture")]
    pub picture: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "CCPBEFMACNF")]
    pub ccpbefmacnf: i64,
}
