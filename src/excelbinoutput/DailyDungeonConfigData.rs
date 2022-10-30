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

pub type DailyDungeonConfigData = Vec<DailyDungeonConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DailyDungeonConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "monday")]
    pub monday: Vec<i64>,

    #[serde(rename = "tuesday")]
    pub tuesday: Vec<i64>,

    #[serde(rename = "wednesday")]
    pub wednesday: Vec<i64>,

    #[serde(rename = "thursday")]
    pub thursday: Vec<i64>,

    #[serde(rename = "friday")]
    pub friday: Vec<i64>,

    #[serde(rename = "saturday")]
    pub saturday: Vec<i64>,

    #[serde(rename = "sunday")]
    pub sunday: Vec<i64>,
}
