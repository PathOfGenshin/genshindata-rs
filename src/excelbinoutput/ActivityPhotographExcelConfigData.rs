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

pub type ActivityPhotographExcelConfigData = Vec<ActivityPhotographExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPhotographExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "CFJMCGFIBKF")]
    pub cfjmcgfibkf: Vec<i64>,

    #[serde(rename = "AKKKJGLAMOF")]
    pub akkkjglamof: Vec<i64>,
}
