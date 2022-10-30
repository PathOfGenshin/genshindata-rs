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

pub type MichiaeDarkChallengeExcelConfigData = Vec<MichiaeDarkChallengeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MichiaeDarkChallengeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "challengeId")]
    pub challenge_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: Vec<i64>,

    #[serde(rename = "groupId")]
    pub group_id: i64,
}
