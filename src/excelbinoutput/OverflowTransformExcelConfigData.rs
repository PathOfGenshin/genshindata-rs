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

pub type OverflowTransformExcelConfigData = Vec<OverflowTransformExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OverflowTransformExcelConfigDatum {
    #[serde(rename = "transformType")]
    pub transform_type: String,

    #[serde(rename = "transformId")]
    pub transform_id: i64,

    #[serde(rename = "transformBaseCount")]
    pub transform_base_count: i64,

    #[serde(rename = "transformResults")]
    pub transform_results: Vec<TransformResult>,

    #[serde(rename = "transformItemLimitType")]
    pub transform_item_limit_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransformResult {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}
