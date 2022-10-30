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

pub type ExpeditionPathExcelConfigData = Vec<ExpeditionPathExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExpeditionPathExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "difficultyId")]
    pub difficulty_id: i64,

    #[serde(rename = "superElement")]
    pub super_element: String,

    #[serde(rename = "basicRewardId")]
    pub basic_reward_id: i64,

    #[serde(rename = "bonusRewardId")]
    pub bonus_reward_id: i64,

    #[serde(rename = "pathNameTextMapHash")]
    pub path_name_text_map_hash: i64,

    #[serde(rename = "pathDescTextMapHash")]
    pub path_desc_text_map_hash: i64,

    #[serde(rename = "AHJEEPHGEOH")]
    pub ahjeephgeoh: Vec<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,
}
