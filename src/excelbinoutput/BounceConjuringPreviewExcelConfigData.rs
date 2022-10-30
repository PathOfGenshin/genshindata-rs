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

pub type BounceConjuringPreviewExcelConfigData = Vec<BounceConjuringPreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BounceConjuringPreviewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "activityStayTime")]
    pub activity_stay_time: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "NPJCHAOMOMG")]
    pub npjchaomomg: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "matchID")]
    pub match_id: i64,
}
