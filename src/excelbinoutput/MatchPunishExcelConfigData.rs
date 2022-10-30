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

pub type MatchPunishExcelConfigData = Vec<MatchPunishExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MatchPunishExcelConfigDatum {
    #[serde(rename = "times")]
    pub times: i64,

    #[serde(rename = "punishTime")]
    pub punish_time: i64,
}
