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

pub type FlightActivityMedalExcelConfigData = Vec<FlightActivityMedalExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FlightActivityMedalExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "medalIcon")]
    pub medal_icon: String,

    #[serde(rename = "dailyInfo")]
    pub daily_info: Vec<DailyInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct DailyInfo {
    #[serde(rename = "watcher")]
    pub watcher: i64,
}
