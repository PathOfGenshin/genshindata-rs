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

pub type PushTipsCodexExcelConfigData = Vec<PushTipsCodexExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PushTipsCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "PushTipId")]
    pub push_tip_id: i64,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,
}
