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

pub type FindHilichurlExcelConfigData = Vec<FindHilichurlExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FindHilichurlExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "guideQuestId")]
    pub guide_quest_id: i64,

    #[serde(rename = "endQuestId")]
    pub end_quest_id: i64,

    #[serde(rename = "assignmentIdList")]
    pub assignment_id_list: Vec<i64>,

    #[serde(rename = "HiliWeiIdList")]
    pub hili_wei_id_list: Vec<i64>,
}
