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

pub type MoonfinTrialLevelExcelConfigData = Vec<MoonfinTrialLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MoonfinTrialLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "LFHBNHPDLHD")]
    pub lfhbnhpdlhd: Vec<i64>,

    #[serde(rename = "mainQuest")]
    pub main_quest: Option<i64>,

    #[serde(rename = "ICOADJELPBO")]
    pub icoadjelpbo: Vec<f64>,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "descriptionTextMapHash")]
    pub description_text_map_hash: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "AHNJPLCJGMP")]
    pub ahnjplcjgmp: Option<String>,

    #[serde(rename = "FBAEGDBCHBN")]
    pub fbaegdbchbn: Option<i64>,

    #[serde(rename = "galleryId")]
    pub gallery_id: Option<i64>,

    #[serde(rename = "challengeId")]
    pub challenge_id: Option<i64>,

    #[serde(rename = "HJNNGKNEFNA")]
    pub hjnngknefna: Option<i64>,

    #[serde(rename = "GFONAHPKIDO")]
    pub gfonahpkido: Option<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}
