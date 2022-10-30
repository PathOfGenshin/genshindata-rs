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

pub type WindFieldShowChallengeExcelConfigData = Vec<WindFieldShowChallengeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WindFieldShowChallengeExcelConfigDatum {
    #[serde(rename = "BMBPGLCFGKD")]
    pub bmbpglcfgkd: i64,

    #[serde(rename = "LFOLKMHENDI")]
    pub lfolkmhendi: Option<i64>,

    #[serde(rename = "showType")]
    pub show_type: String,

    #[serde(rename = "DNIEBODAFAA")]
    pub dniebodafaa: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "HEKDEKNHFGI")]
    pub hekdeknhfgi: i64,

    #[serde(rename = "EFDFKHOHKLD")]
    pub efdfkhohkld: i64,

    #[serde(rename = "LKBHFCIPKJJ")]
    pub lkbhfcipkjj: Option<i64>,
}
