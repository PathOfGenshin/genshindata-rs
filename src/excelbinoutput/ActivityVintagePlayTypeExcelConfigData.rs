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

pub type ActivityVintagePlayTypeExcelConfigData = Vec<ActivityVintagePlayTypeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintagePlayTypeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "DGGMIFJMJBP")]
    pub dggmifjmjbp: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "LCPLPHEEAAL")]
    pub lcplpheeaal: Option<i64>,

    #[serde(rename = "guideQuestId")]
    pub guide_quest_id: Option<i64>,

    #[serde(rename = "tutorialId")]
    pub tutorial_id: Option<i64>,
}
