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

pub type TreeTypeExcelConfigData = Vec<TreeTypeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TreeTypeExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "treePattern")]
    pub tree_pattern: String,

    #[serde(rename = "treeType")]
    pub tree_type: String,
}
