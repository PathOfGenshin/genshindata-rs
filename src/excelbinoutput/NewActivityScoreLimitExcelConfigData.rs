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

pub type NewActivityScoreLimitExcelConfigData = Vec<NewActivityScoreLimitExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityScoreLimitExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "limitValue")]
    pub limit_value: i64,
}
