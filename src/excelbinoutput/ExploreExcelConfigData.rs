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

pub type ExploreExcelConfigData = Vec<ExploreExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExploreExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "exp")]
    pub exp: i64,
}
