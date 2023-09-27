/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReminderExcelConfigData = Vec<ReminderExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderExcelConfigDatum {
    pub id: i64,
    pub speaker_text_map_hash: i64,
    pub content_text_map_hash: i64,
    pub show_time: Option<f64>,
    pub next_reminder_id: Option<i64>,
    pub sound_effect: String,
    pub has_audio: Option<bool>,
    #[serde(rename = "IIPODMBIKJH")]
    pub iipodmbikjh: String,
    pub delay: Option<f64>,
    pub style: Option<Style>,
    #[serde(rename = "BPOBAPILOEE")]
    pub bpobapiloee: Option<i64>,
    #[serde(rename = "GPCJCBLIGHJ")]
    pub gpcjcblighj: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Style {
    Banner,
    #[serde(rename = "DialogueWithPortrait")]
    DialogueWithPortrait,
    #[serde(rename = "EventPromptDown")]
    EventPromptDown,
    #[serde(rename = "InfoTextDialog")]
    InfoTextDialog,
    #[serde(rename = "PenumbraInfo")]
    PenumbraInfo,
    #[serde(rename = "PenumbraMiniStory")]
    PenumbraMiniStory,
    #[serde(rename = "PenumbraStory")]
    PenumbraStory,
    #[serde(rename = "PenumbraTarget")]
    PenumbraTarget,
    #[serde(rename = "WhiteMessage")]
    WhiteMessage,
}
