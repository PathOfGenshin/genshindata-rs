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

pub type CityTaskOpenExcelConfigData = Vec<CityTaskOpenExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CityTaskOpenExcelConfigDatum {
    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "BHBDKOMLIDN")]
    pub bhbdkomlidn: Vec<i64>,
}
