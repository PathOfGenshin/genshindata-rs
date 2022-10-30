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

pub type RogueCellWeightExcelConfigData = Vec<RogueCellWeightExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueCellWeightExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CGKKLOGJGFA")]
    pub cgkklogjgfa: i64,

    #[serde(rename = "LNFFFGFKHBG")]
    pub lnfffgfkhbg: i64,

    #[serde(rename = "CIJDFAEFFCN")]
    pub cijdfaeffcn: i64,
}
