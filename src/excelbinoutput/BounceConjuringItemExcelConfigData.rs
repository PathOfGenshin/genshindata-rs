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

pub type BounceConjuringItemExcelConfigData = Vec<BounceConjuringItemExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BounceConjuringItemExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "bgIconHashSuffix")]
    pub bg_icon_hash_suffix: i64,

    #[serde(rename = "bgIconHashPre")]
    pub bg_icon_hash_pre: i64,
}
