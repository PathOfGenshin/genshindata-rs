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

pub type ActivitySpiceExcelConfigData = Vec<ActivitySpiceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySpiceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "PKIIGKFDMEK")]
    pub pkiigkfdmek: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "FMAKLIDDPAA")]
    pub fmakliddpaa: i64,

    #[serde(rename = "HLCKNKJIKCD")]
    pub hlcknkjikcd: i64,

    #[serde(rename = "JHOCOHDPDAH")]
    pub jhocohdpdah: i64,
}
