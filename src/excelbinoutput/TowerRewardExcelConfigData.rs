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

pub type TowerRewardExcelConfigData = Vec<TowerRewardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TowerRewardExcelConfigDatum {
    #[serde(rename = "IEPBHNILIDB")]
    pub iepbhnilidb: i64,

    #[serde(rename = "DGKHCGBPFDM")]
    pub dgkhcgbpfdm: i64,

    #[serde(rename = "BCOFGJBABGD")]
    pub bcofgjbabgd: i64,

    #[serde(rename = "CAEOHANFNAP")]
    pub caeohanfnap: i64,

    #[serde(rename = "DGJDIBJKGMI")]
    pub dgjdibjkgmi: i64,

    #[serde(rename = "MHDLDDFOIJD")]
    pub mhdlddfoijd: Vec<i64>,
}
