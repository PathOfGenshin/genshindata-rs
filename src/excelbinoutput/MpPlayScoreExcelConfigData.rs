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

pub type MpPlayScoreExcelConfigData = Vec<MpPlayScoreExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MpPlayScoreExcelConfigDatum {
    #[serde(rename = "playType")]
    pub play_type: String,

    #[serde(rename = "param")]
    pub param: Vec<i64>,

    #[serde(rename = "isCalcScore")]
    pub is_calc_score: Option<bool>,
}
