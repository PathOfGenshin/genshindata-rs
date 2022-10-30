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

pub type MichiaeBossChallengeExcelConfigData = Vec<MichiaeBossChallengeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MichiaeBossChallengeExcelConfigDatum {
    #[serde(rename = "levelID")]
    pub level_id: i64,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "groupID")]
    pub group_id: i64,
}
