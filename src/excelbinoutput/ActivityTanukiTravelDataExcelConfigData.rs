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

pub type ActivityTanukiTravelDataExcelConfigData = Vec<ActivityTanukiTravelDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityTanukiTravelDataExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "guideQuestId")]
    pub guide_quest_id: i64,

    #[serde(rename = "BHKPDHECMPK")]
    pub bhkpdhecmpk: Vec<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "BFMAFNJPPHD")]
    pub bfmafnjpphd: i64,

    #[serde(rename = "CBPBJALLBGN")]
    pub cbpbjallbgn: i64,

    #[serde(rename = "IOBPNLHKFAP")]
    pub iobpnlhkfap: Vec<i64>,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,
}
