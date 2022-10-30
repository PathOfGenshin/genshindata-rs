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

pub type ActivityDeliveryExcelConfigData = Vec<ActivityDeliveryExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityDeliveryExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "dailyConfigIdList")]
    pub daily_config_id_list: Vec<i64>,

    #[serde(rename = "needPlayerLevel")]
    pub need_player_level: i64,
}
