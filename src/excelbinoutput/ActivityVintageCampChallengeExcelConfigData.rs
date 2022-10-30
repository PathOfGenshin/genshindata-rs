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

pub type ActivityVintageCampChallengeExcelConfigData =
    Vec<ActivityVintageCampChallengeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintageCampChallengeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CNGBDMFLGGF")]
    pub cngbdmflggf: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "KLJLLCKBJGF")]
    pub kljllckbjgf: i64,

    #[serde(rename = "NKMBOEBPONE")]
    pub nkmboebpone: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "CEFGGOMEFBI")]
    pub cefggomefbi: Vec<String>,

    #[serde(rename = "AOEBEJKFEBA")]
    pub aoebejkfeba: Vec<i64>,

    #[serde(rename = "PLHNOFMNEHM")]
    pub plhnofmnehm: String,

    #[serde(rename = "BPONEFHCAFP")]
    pub bponefhcafp: Vec<i64>,

    #[serde(rename = "IAPJAOPEIOP")]
    pub iapjaopeiop: i64,

    #[serde(rename = "CPOAMLCBFAG")]
    pub cpoamlcbfag: i64,
}
