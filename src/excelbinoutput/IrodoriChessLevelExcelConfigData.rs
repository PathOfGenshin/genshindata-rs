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

pub type IrodoriChessLevelExcelConfigData = Vec<IrodoriChessLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct IrodoriChessLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "BAHFEFHGMGK")]
    pub bahfefhgmgk: i64,

    #[serde(rename = "KBMADIGNJFG")]
    pub kbmadignjfg: i64,

    #[serde(rename = "ALPLOBLPOLO")]
    pub alploblpolo: i64,
}
