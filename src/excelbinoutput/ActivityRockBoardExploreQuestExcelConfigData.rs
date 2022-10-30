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

pub type ActivityRockBoardExploreQuestExcelConfigData =
    Vec<ActivityRockBoardExploreQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityRockBoardExploreQuestExcelConfigDatum {
    #[serde(rename = "MGNHACIGAJJ")]
    pub mgnhacigajj: i64,

    #[serde(rename = "questID")]
    pub quest_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "iconName")]
    pub icon_name: String,

    #[serde(rename = "DGGMIFJMJBP")]
    pub dggmifjmjbp: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,
}
