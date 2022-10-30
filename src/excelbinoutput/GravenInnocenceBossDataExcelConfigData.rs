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

pub type GravenInnocenceBossDataExcelConfigData = Vec<GravenInnocenceBossDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GravenInnocenceBossDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "GBJADHAELIL")]
    pub gbjadhaelil: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "ECJHCEIEBEC")]
    pub ecjhceiebec: i64,
}
