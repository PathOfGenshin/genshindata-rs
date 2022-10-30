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

pub type ReminderExcelConfigData = Vec<ReminderExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReminderExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "speakerTextMapHash")]
    pub speaker_text_map_hash: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "showTime")]
    pub show_time: Option<f64>,

    #[serde(rename = "nextReminderId")]
    pub next_reminder_id: Option<i64>,

    #[serde(rename = "soundEffect")]
    pub sound_effect: String,

    #[serde(rename = "hasAudio")]
    pub has_audio: Option<bool>,

    #[serde(rename = "delay")]
    pub delay: Option<f64>,

    #[serde(rename = "style")]
    pub style: Option<Style>,
}

#[derive(Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "Banner")]
    Banner,

    #[serde(rename = "EventPromptDown")]
    EventPromptDown,

    #[serde(rename = "InfoTextDialog")]
    InfoTextDialog,

    #[serde(rename = "WhiteMessage")]
    WhiteMessage,
}
