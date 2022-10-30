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

pub type CustomLevelUiConfigData = Vec<CustomLevelUiConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CustomLevelUiConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: Option<i64>,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "CLMBMJMHOCD")]
    pub clmbmjmhocd: Vec<i64>,
}
