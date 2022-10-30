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

pub type EchoShellRewardExcelConfigData = Vec<EchoShellRewardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EchoShellRewardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "EEFPALNMLLC")]
    pub eefpalnmllc: i64,

    #[serde(rename = "HFEPHNHPPKJ")]
    pub hfephnhppkj: Option<bool>,
}