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

pub type TrialReliquaryExcelConfigData = Vec<TrialReliquaryExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TrialReliquaryExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "ReliquaryId")]
    pub reliquary_id: i64,

    #[serde(rename = "Level")]
    pub level: i64,

    #[serde(rename = "MainPropId")]
    pub main_prop_id: i64,

    #[serde(rename = "AppendPropList")]
    pub append_prop_list: Vec<i64>,
}
