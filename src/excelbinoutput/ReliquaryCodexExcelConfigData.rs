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

pub type ReliquaryCodexExcelConfigData = Vec<ReliquaryCodexExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReliquaryCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "suitId")]
    pub suit_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "cupId")]
    pub cup_id: Option<i64>,

    #[serde(rename = "leatherId")]
    pub leather_id: Option<i64>,

    #[serde(rename = "capId")]
    pub cap_id: i64,

    #[serde(rename = "flowerId")]
    pub flower_id: Option<i64>,

    #[serde(rename = "sandId")]
    pub sand_id: Option<i64>,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,
}
