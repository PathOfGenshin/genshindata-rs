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

pub type ActivityShopOverallExcelConfigData = Vec<ActivityShopOverallExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityShopOverallExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "shopType")]
    pub shop_type: String,

    #[serde(rename = "sheetList")]
    pub sheet_list: Vec<i64>,
}
