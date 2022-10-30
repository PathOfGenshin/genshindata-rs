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

pub type LuminanceStoneChallengeOverallExcelConfigData =
    Vec<LuminanceStoneChallengeOverallExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LuminanceStoneChallengeOverallExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "OCFEKJMLKPE")]
    pub ocfekjmlkpe: i64,

    #[serde(rename = "PHNPHHGFAGF")]
    pub phnphhgfagf: i64,

    #[serde(rename = "HHMKDOMILAL")]
    pub hhmkdomilal: Vec<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "HOBLCBBCMKE")]
    pub hoblcbbcmke: i64,
}
