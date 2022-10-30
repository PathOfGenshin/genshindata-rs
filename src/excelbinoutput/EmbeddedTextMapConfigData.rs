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

pub type EmbeddedTextMapConfigData = Vec<EmbeddedTextMapConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EmbeddedTextMapConfigDatum {
    #[serde(rename = "textMapId")]
    pub text_map_id: String,

    #[serde(rename = "textMapContentTextMapHash")]
    pub text_map_content_text_map_hash: i64,
}
