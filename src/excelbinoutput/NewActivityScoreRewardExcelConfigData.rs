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

pub type NewActivityScoreRewardExcelConfigData = Vec<NewActivityScoreRewardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityScoreRewardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,

    #[serde(rename = "score")]
    pub score: i64,

    #[serde(rename = "activityScoreTipsTextMapHash")]
    pub activity_score_tips_text_map_hash: i64,

    #[serde(rename = "activityScoreExtraTipsTextMapHash")]
    pub activity_score_extra_tips_text_map_hash: i64,
}
