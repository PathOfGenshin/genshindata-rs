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

pub type ExpeditionActivityMarkerExcelConfigData = Vec<ExpeditionActivityMarkerExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExpeditionActivityMarkerExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "posX")]
    pub pos_x: f64,

    #[serde(rename = "posY")]
    pub pos_y: f64,

    #[serde(rename = "pictureHashPre")]
    pub picture_hash_pre: i64,

    #[serde(rename = "pictureHashSuffix")]
    pub picture_hash_suffix: i64,
}
