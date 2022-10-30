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

pub type BonusTreasureSolutionExcelConfigData = Vec<BonusTreasureSolutionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BonusTreasureSolutionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LGENGDLHNHK")]
    pub lgengdlhnhk: i64,

    #[serde(rename = "MFFEJAAEHBF")]
    pub mffejaaehbf: i64,

    #[serde(rename = "NABAAPJNECF")]
    pub nabaapjnecf: Vec<i64>,
}
