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

pub type BlessingScanTypeExcelConfigData = Vec<BlessingScanTypeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BlessingScanTypeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "typeId")]
    pub type_id: i64,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "upPoolId")]
    pub up_pool_id: Vec<i64>,

    #[serde(rename = "dailyScanLimit")]
    pub daily_scan_limit: i64,

    #[serde(rename = "typeNameTextMapHash")]
    pub type_name_text_map_hash: i64,

    #[serde(rename = "typeNameWithColorTextMapHash")]
    pub type_name_with_color_text_map_hash: i64,
}
