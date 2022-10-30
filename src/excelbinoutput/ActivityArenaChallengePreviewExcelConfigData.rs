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

pub type ActivityArenaChallengePreviewExcelConfigData =
    Vec<ActivityArenaChallengePreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityArenaChallengePreviewExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "centerPosList")]
    pub center_pos_list: Vec<f64>,

    #[serde(rename = "guideQuestId1")]
    pub guide_quest_id1: i64,

    #[serde(rename = "OPIFMHAGBPD")]
    pub opifmhagbpd: Option<i64>,

    #[serde(rename = "NBDHGAKNLPM")]
    pub nbdhgaknlpm: Option<i64>,
}
