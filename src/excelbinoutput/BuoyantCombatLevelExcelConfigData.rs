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

pub type BuoyantCombatLevelExcelConfigData = Vec<BuoyantCombatLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BuoyantCombatLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: i64,

    #[serde(rename = "GEFICGMHOGK")]
    pub geficgmhogk: i64,

    #[serde(rename = "OKEGJABPGAD")]
    pub okegjabpgad: i64,

    #[serde(rename = "LFHBNHPDLHD")]
    pub lfhbnhpdlhd: Vec<i64>,

    #[serde(rename = "JDOELJDKKDJ")]
    pub jdoeljdkkdj: Vec<i64>,

    #[serde(rename = "recommendLevel")]
    pub recommend_level: Vec<i64>,

    #[serde(rename = "JCDDHAOJGKL")]
    pub jcddhaojgkl: Vec<f64>,
}
