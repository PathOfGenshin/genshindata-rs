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

pub type DocumentExcelConfigData = Vec<DocumentExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DocumentExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentLocalizedId")]
    pub content_localized_id: Option<i64>,

    #[serde(rename = "previewPath")]
    pub preview_path: String,

    #[serde(rename = "videoPath")]
    pub video_path: VideoPath,

    #[serde(rename = "documentType")]
    pub document_type: Option<String>,

    #[serde(rename = "subtitleID")]
    pub subtitle_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum VideoPath {
    #[serde(rename = "Ambor_Readings.usm")]
    AmborReadingsUsm,

    #[serde(rename = "")]
    Empty,
}
