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

pub type ActivityPotionModeChoiceExcelConfigData = Vec<ActivityPotionModeChoiceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPotionModeChoiceExcelConfigDatum {
    #[serde(rename = "KBDPPNIEECG")]
    pub kbdppnieecg: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "DKHKDONKPOF")]
    pub dkhkdonkpof: i64,
}
