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

pub type ExpeditionChallengeExcelConfigData = Vec<ExpeditionChallengeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExpeditionChallengeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "rewardChallengeIndex")]
    pub reward_challenge_index: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "challengeNameTextMapHash")]
    pub challenge_name_text_map_hash: i64,

    #[serde(rename = "challengeDescTextMapHash")]
    pub challenge_desc_text_map_hash: i64,

    #[serde(rename = "superElement")]
    pub super_element: String,

    #[serde(rename = "centerPosList")]
    pub center_pos_list: Vec<f64>,

    #[serde(rename = "centerRadius")]
    pub center_radius: i64,

    #[serde(rename = "unlockTime")]
    pub unlock_time: Option<i64>,
}
