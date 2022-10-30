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

pub type ReliquaryMainPropExcelConfigData = Vec<ReliquaryMainPropExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReliquaryMainPropExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "propDepotId")]
    pub prop_depot_id: i64,

    #[serde(rename = "propType")]
    pub prop_type: String,

    #[serde(rename = "affixName")]
    pub affix_name: String,
}
