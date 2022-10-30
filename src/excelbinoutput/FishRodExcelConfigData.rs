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

pub type FishRodExcelConfigData = Vec<FishRodExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FishRodExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "FAHBHJLFFKI")]
    pub fahbhjlffki: f64,

    #[serde(rename = "cityId")]
    pub city_id: Option<i64>,

    #[serde(rename = "LBCENHBFCDO")]
    pub lbcenhbfcdo: Option<f64>,

    #[serde(rename = "PKEBFAABOHG")]
    pub pkebfaabohg: Option<f64>,

    #[serde(rename = "AJEFMFFFDFK")]
    pub ajefmfffdfk: Option<f64>,
}
