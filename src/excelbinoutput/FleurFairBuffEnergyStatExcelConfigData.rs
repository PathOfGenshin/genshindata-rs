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

pub type FleurFairBuffEnergyStatExcelConfigData = Vec<FleurFairBuffEnergyStatExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FleurFairBuffEnergyStatExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "statParam")]
    pub stat_param: String,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,
}
