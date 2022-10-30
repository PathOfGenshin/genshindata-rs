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

pub type EmotionTemplateExcelConfigData = Vec<EmotionTemplateExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EmotionTemplateExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "freeStyleId")]
    pub free_style_id: i64,

    #[serde(rename = "bubbleName")]
    pub bubble_name: String,

    #[serde(rename = "emotionPath")]
    pub emotion_path: String,

    #[serde(rename = "protectTime")]
    pub protect_time: f64,
}
