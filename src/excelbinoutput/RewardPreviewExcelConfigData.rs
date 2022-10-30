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

pub type RewardPreviewExcelConfigData = Vec<RewardPreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RewardPreviewExcelConfigDatum {
    #[serde(rename = "Desc")]
    pub desc: String,

    #[serde(rename = "previewItems")]
    pub preview_items: Vec<PreviewItem>,

    #[serde(rename = "id")]
    pub id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PreviewItem {
    #[serde(rename = "count")]
    pub count: String,

    #[serde(rename = "id")]
    pub id: Option<i64>,
}
