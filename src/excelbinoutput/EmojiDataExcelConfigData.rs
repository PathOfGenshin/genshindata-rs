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

pub type EmojiDataExcelConfigData = Vec<EmojiDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EmojiDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "setID")]
    pub set_id: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,
}
