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

pub type SalvageOverAllExcelConfigData = Vec<SalvageOverAllExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SalvageOverAllExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "BDBJLBCLGPC")]
    pub bdbjlbclgpc: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "guideQuestId")]
    pub guide_quest_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "regionRadius")]
    pub region_radius: i64,

    #[serde(rename = "JPMPCJDLMON")]
    pub jpmpcjdlmon: i64,

    #[serde(rename = "reminderId")]
    pub reminder_id: i64,

    #[serde(rename = "HLCAEHIHLFM")]
    pub hlcaehihlfm: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "JBPBFNCDBHK")]
    pub jbpbfncdbhk: i64,

    #[serde(rename = "FEJKLIANLPM")]
    pub fejklianlpm: i64,
}
