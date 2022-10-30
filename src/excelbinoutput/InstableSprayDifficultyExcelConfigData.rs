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

pub type InstableSprayDifficultyExcelConfigData = Vec<InstableSprayDifficultyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InstableSprayDifficultyExcelConfigDatum {
    #[serde(rename = "LKNFKIGLALE")]
    pub lknfkiglale: i64,

    #[serde(rename = "MDFNAHJMPHI")]
    pub mdfnahjmphi: i64,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "LIELAIPEDMJ")]
    pub lielaipedmj: i64,
}
